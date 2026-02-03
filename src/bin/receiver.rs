use Blue_Fox::transport::receiver::Receiver;
use Blue_Fox::crypto::handshake::perform_handshake;
fn main() {
    //let handshake = perform_handshake();
    let session_key = [42u8; 32];

    let mut receiver = Receiver::new(
        "127.0.0.1:50001",
        "received.bin",
        session_key,
    ).expect("receiver init failed");

    receiver.run().unwrap();
}
