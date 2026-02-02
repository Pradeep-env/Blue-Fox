use std::net::{UdpSocket, SocketAddr};
use std::io;

pub fn send_packet(
    socket: &UdpSocket,
    target: SocketAddr,
    data: &[u8],
) -> io::Result<()> {
    socket.send_to(data, target)?;
    Ok(())
}

pub fn recv_packet(
    socket: &UdpSocket,
    buf: &mut [u8],
) -> io::Result<(usize, SocketAddr)> {
    socket.recv_from(buf)
}
