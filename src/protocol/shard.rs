#[derive(Debug, Copy, Clone)]
pub struct ShardHeader {
    pub session_id: u32,
    pub chunk_seq: u32,
    pub payload_len: u16,
    pub shard_index: u8,
    pub total_shards: u8,
}

impl ShardHeader {
    pub fn to_bytes(&self) -> [u8; 12] {
        let mut buf = [0u8; 12];
        buf[0..4].copy_from_slice(&self.session_id.to_be_bytes());
        buf[4..8].copy_from_slice(&self.chunk_seq.to_be_bytes());
        buf[8..10].copy_from_slice(&self.payload_len.to_be_bytes());
        buf[10] = self.shard_index;
        buf[11] = self.total_shards;
        buf
    }

    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {
            session_id: u32::from_be_bytes(buf[0..4].try_into().unwrap()),
            chunk_seq: u32::from_be_bytes(buf[4..8].try_into().unwrap()),
            payload_len: u16::from_be_bytes(buf[8..10].try_into().unwrap()),
            shard_index: buf[10],
            total_shards: buf[11],
        }
    }
}
