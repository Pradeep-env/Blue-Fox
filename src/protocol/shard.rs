#[derive(Debug, Copy, Clone)]
pub struct ShardHeader {
    pub session_id: u32,
    pub chunk_seq: u32,
    pub shard_index: u8,
    pub total_shards: u8,
}

impl ShardHeader {
    pub fn to_bytes(&self) -> [u8; 10] {
        let mut buf = [0u8; 10];
        buf[0..4].copy_from_slice(&self.session_id.to_be_bytes());
        buf[4..8].copy_from_slice(&self.chunk_seq.to_be_bytes());
        buf[8] = self.shard_index;
        buf[9] = self.total_shards;
        buf
    }
}
