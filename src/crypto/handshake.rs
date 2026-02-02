use x25519_dalek::{EphemeralSecret, PublicKey};
use rand::rngs::OsRng;
use hkdf::Hkdf;
use sha2::Sha256;

pub struct HandshakeResult {
    pub public_key: [u8; 32],
    pub session_key: [u8; 32],
}

pub fn perform_handshake() -> HandshakeResult {
    let secret = EphemeralSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);

    // Loopback for now (self handshake)
    let peer_public = public;

    let shared_secret = secret.diffie_hellman(&peer_public);

    let hk = Hkdf::<Sha256>::new(None, shared_secret.as_bytes());
    let mut session_key = [0u8; 32];
    hk.expand(b"local-messenger-session", &mut session_key)
        .expect("HKDF expand failed");

    HandshakeResult {
        public_key: *public.as_bytes(),
        session_key,
    }
}
