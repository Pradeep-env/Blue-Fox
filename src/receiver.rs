mod crypto;
mod protocol;
mod transport;
mod fec;

use crypto::handshake::perform_handshake;
use transport::receiver::Receiver;

fn main() {
    let handshake = perform_handshake();

    let mut receiver = Receiver::new(
        "127.0.0.1:50001",
        "received.bin",
        handshake.session_key,
    ).expect("receiver init failed");

    receiver.run().unwrap();
}
