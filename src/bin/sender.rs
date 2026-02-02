use Blue_Fox::crypto::handshake::perform_handshake;
use Blue_Fox::crypto::aead::encrypt;
use Blue_Fox::protocol::chunk::ChunkReader;
use Blue_Fox::protocol::packet::{PacketHeader, PacketType, PROTOCOL_VERSION};
use Blue_Fox::protocol::constants::{DATA_SHARDS, PARITY_SHARDS};
use Blue_Fox::protocol::shard::ShardHeader;
use Blue_Fox::fec::reed_solomon::RsCodec;
use Blue_Fox::transport::udp::send_packet;

use std::net::UdpSocket;


fn main() {
    let handshake = perform_handshake();

    let socket =
        UdpSocket::bind("127.0.0.1:50000").expect("bind sender");

    let target = "127.0.0.1:50001".parse().unwrap();

    let mut reader =
        ChunkReader::new("testfile.bin").expect("file open failed");

    let rs = RsCodec::new(DATA_SHARDS, PARITY_SHARDS);
    let session_id = 0xfeedface;

    while let Some((seq_no, chunk)) =
        reader.next_chunk().expect("read failed")
    {
        let header = PacketHeader {
            version: PROTOCOL_VERSION,
            packet_type: PacketType::Data,
            session_id,
            seq_no,
            payload_len: chunk.len() as u16,
        };

        let aad = header.to_bytes();

        let mut nonce = [0u8; 12];
        nonce[8..12].copy_from_slice(&seq_no.to_be_bytes());

        let ciphertext = encrypt(
            &handshake.session_key,
            &nonce,
            &chunk,
            &aad,
        ).expect("encrypt failed");

        let shard_size =
            (ciphertext.len() + DATA_SHARDS - 1) / DATA_SHARDS;

        let mut shards = Vec::new();

        for i in 0..DATA_SHARDS {
            let start = i * shard_size;
            let end = ((i + 1) * shard_size).min(ciphertext.len());

            let mut shard = vec![0u8; shard_size];
            if start < ciphertext.len() {
                shard[..end - start]
                    .copy_from_slice(&ciphertext[start..end]);
            }
            shards.push(shard);
        }

        for _ in 0..PARITY_SHARDS {
            shards.push(vec![0u8; shard_size]);
        }

        rs.encode(&mut shards);

        let total_shards = shards.len() as u8;

        for (i, shard) in shards.iter().enumerate() {
            let shard_header = ShardHeader {
                session_id,
                chunk_seq: seq_no,
                shard_index: i as u8,
                total_shards,
            };

            let mut packet = Vec::new();
            packet.extend_from_slice(&shard_header.to_bytes());
            packet.extend_from_slice(shard);

            send_packet(&socket, target, &packet).unwrap();
        }

        println!("Sent chunk {}", seq_no);
    }
}
