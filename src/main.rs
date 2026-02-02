mod crypto;
mod protocol;
mod transport;
mod fec;

use crypto::handshake::perform_handshake;
use crypto::aead::{encrypt, decrypt};
use protocol::packet::{PacketHeader, PacketType, PROTOCOL_VERSION};
use fec::reed_solomon::RsCodec;

fn main() {
    // === Setup ===
    let handshake = perform_handshake();

    let header = PacketHeader {
        version: PROTOCOL_VERSION,
        packet_type: PacketType::Data,
        session_id: 0x12345678,
        seq_no: 42,
        payload_len: 0, // filled later
    };

    let plaintext = b"this message must survive packet loss";

    // Nonce = 8 zero bytes + seq_no
    let mut nonce = [0u8; 12];
    nonce[8..12].copy_from_slice(&header.seq_no.to_be_bytes());

    let aad = header.to_bytes();

    // === Encrypt ===
    let ciphertext = encrypt(
        &handshake.session_key,
        &nonce,
        plaintext,
        &aad,
    ).expect("encrypt failed");

    // === RS encode ===
    let data_shards = 4;
    let parity_shards = 2;
    let codec = RsCodec::new(data_shards, parity_shards);

    let shard_size =
        (ciphertext.len() + data_shards - 1) / data_shards;

    let mut shards: Vec<Vec<u8>> = Vec::new();

    for i in 0..data_shards {
        let start = i * shard_size;
        let end = ((i + 1) * shard_size).min(ciphertext.len());

        let mut shard = vec![0u8; shard_size];
        if start < ciphertext.len() {
            shard[..end - start]
                .copy_from_slice(&ciphertext[start..end]);
        }
        shards.push(shard);
    }

    // Add parity shards
    for _ in 0..parity_shards {
        shards.push(vec![0u8; shard_size]);
    }

    codec.encode(&mut shards);

    // === Simulate packet loss ===
    let mut received: Vec<Option<Vec<u8>>> =
        shards.into_iter().map(Some).collect();

    received[1] = None; // lose data shard
    received[4] = None; // lose parity shard

    // === Reconstruct ===
    let ok = codec.reconstruct(&mut received);
    assert!(ok, "RS reconstruction failed");

    // === Reassemble ciphertext ===
    let mut recovered_ciphertext = Vec::new();
    for shard in received.iter().take(data_shards) {
        recovered_ciphertext.extend_from_slice(
            shard.as_ref().unwrap(),
        );
    }
    recovered_ciphertext.truncate(ciphertext.len());

    // === Decrypt ===
    let decrypted = decrypt(
        &handshake.session_key,
        &nonce,
        &recovered_ciphertext,
        &aad,
    ).expect("decrypt failed");

    println!(
        "Recovered message: {}",
        String::from_utf8_lossy(&decrypted)
    );
}
