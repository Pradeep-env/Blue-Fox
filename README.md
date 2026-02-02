# Blue-Fox
Aiming to develop a open-source offiline messenger and file share app.


## What am I aming to solve vs What is already existing.

| App                      | Offline P2P | Large files | High-speed bulk transfer | Mesh routing |
| ------------------------ | ----------- | ----------- | ------------------------ | ------------ |
| Bridgefy                 | Yes         | No          | Low                      | Yes          |
| Signal Offline / AirChat | Yes         | Limited     | Low-Moderate             | No           |
| Briar                    | Yes         | Limited     | Low                      | Yes          |
| Bitchat                  | Yes         | No          | Low                      | Yes          |
| Serval Mesh              | Yes         | Limited     | Low                      | Yes          |
| Blue Fox                 | Yes         | Yes         | High                     | Yes          |


## Why choose Wifi Direct mesh over Blutooth mesh for this project

| Property          | Bluetooth Mesh   | Wi-Fi Direct Mesh |
| ----------------- | ---------------- | ----------------- |
| Frequency         | 2.4 GHz          | 2.4 / 5 GHz       |
| Channel width     | ~1–2 MHz         | 20–80 MHz         |
| Modulation        | GFSK / low-order | OFDM              |
| Tx power (phones) | ~10 dBm          | ~18–23 dBm        |
| Antenna use       | Tiny, shared     | Larger, tuned     |


## V1 goals

1. Local offline messenger

2. Text + files + images

3. Single-hop only (no mesh yet)

4. Wi-Fi Direct

5. Encrypted UDP

6. No accounts, no identity persistence 


## Phase 1: Technology decisions (final)

- Transport: UDP

- Discovery: Wi-Fi Direct

- Key exchange: X25519

- KDF: HKDF-SHA256

- AEAD: ChaCha20-Poly1305

- Reliability: Reed–Solomon (erasure coding)

- Chunk size: 64 KB

- RS block size: ~1 MB

- Language (core): Rust

- UI (later): Android (Kotlin) or Flutter


