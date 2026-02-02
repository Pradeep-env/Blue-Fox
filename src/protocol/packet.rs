pub const PROTOCOL_VERSION: u8 = 1;

#[derive(Debug, Copy, Clone)]
pub enum PacketType {
    Hello = 1,
    Data  = 2,
    Fin   = 3,
}

#[derive(Debug)]
pub struct PacketHeader {
    pub version: u8,
    pub packet_type: PacketType,
    pub session_id: u32,
    pub seq_no: u32,
    pub payload_len: u16,
}

impl PacketHeader {
    pub fn to_bytes(&self) -> [u8; 12] {
        let mut buf = [0u8; 12];

        buf[0] = self.version;
        buf[1] = self.packet_type as u8;
        buf[2..6].copy_from_slice(&self.session_id.to_be_bytes());
        buf[6..10].copy_from_slice(&self.seq_no.to_be_bytes());
        buf[10..12].copy_from_slice(&self.payload_len.to_be_bytes());

        buf
    }
}
