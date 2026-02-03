use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::net::UdpSocket;

use crate::crypto::aead::decrypt;
use crate::fec::reed_solomon::RsCodec;
use crate::protocol::packet::{PacketHeader, PacketType, PROTOCOL_VERSION};
use crate::protocol::constants::{DATA_SHARDS, PARITY_SHARDS};

pub struct ChunkBuffer {
    shards: Vec<Option<Vec<u8>>>,
}

pub struct Receiver {
    socket: UdpSocket,
    codec: RsCodec,
    chunks: HashMap<u32, ChunkBuffer>,
    output: File,
    session_key: [u8; 32],
}

impl Receiver {
    pub fn new(
        bind_addr: &str,
        output_path: &str,
        session_key: [u8; 32],
    ) -> io::Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind(bind_addr)?,
            codec: RsCodec::new(DATA_SHARDS, PARITY_SHARDS),
            chunks: HashMap::new(),
            output: File::create(output_path)?,
            session_key,
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut buf = [0u8; 1500];

        loop {
            let (len, _) = self.socket.recv_from(&mut buf)?;

            // --- Parse shard header ---
            use crate::protocol::shard::ShardHeader;

            if len < 12 {
              continue; // malformed packet
            }

            let shard_header = ShardHeader::from_bytes(&buf[..12]);
            let payload = buf[12..len].to_vec();

            if shard_header.total_shards == 0 {
                continue; // invalid packet
            }


            let entry = self.chunks.entry(shard_header.chunk_seq).or_insert_with(|| {
               ChunkBuffer {
                  shards: vec![None; shard_header.total_shards as usize],
               }
            });



            if shard_header.shard_index as usize >= entry.shards.len() {
                 continue; // corrupted packet
            }

            if entry.shards[shard_header.shard_index as usize].is_none() {
                entry.shards[shard_header.shard_index as usize] = Some(payload);
            }


            // Count received shards
            let received =
                entry.shards.iter().filter(|s| s.is_some()).count();

            if received < DATA_SHARDS {
                continue;
            }

            // --- Reconstruct ---
            let mut shards = entry.shards.clone();

            if !self.codec.reconstruct(&mut shards) {
                continue;
            }

            // --- Reassemble ciphertext ---

            let mut ciphertext = Vec::new();
            for shard in shards.iter().take(DATA_SHARDS) {
               ciphertext.extend_from_slice(shard.as_ref().unwrap());
            }

            // --- Build packet header for AAD ---

            let header = PacketHeader {
                version: PROTOCOL_VERSION,
                packet_type: PacketType::Data,
                session_id: shard_header.session_id,
                seq_no: shard_header.chunk_seq,
                payload_len: shard_header.payload_len,
            };



            // IMPORTANT: truncate padding
            let ciphertext_len = header.payload_len as usize + 16; // Poly1305 tag
            ciphertext.truncate(ciphertext_len);


            let aad = header.to_bytes();

            let mut nonce = [0u8; 12];
            nonce[8..12].copy_from_slice(&shard_header.chunk_seq.to_be_bytes());

            let plaintext = decrypt(
                &self.session_key,
                &nonce,
                &ciphertext,
                &aad,
            ).expect("decrypt failed");

            self.output.write_all(&plaintext)?;

            self.chunks.remove(&shard_header.chunk_seq);

            println!("Received chunk {}", shard_header.chunk_seq);
        }
    }
}
