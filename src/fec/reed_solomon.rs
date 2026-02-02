use reed_solomon_erasure::galois_8::ReedSolomon;

pub struct RsCodec {
    rs: ReedSolomon,
    data_shards: usize,
    parity_shards: usize,
}

impl RsCodec {
    pub fn new(data_shards: usize, parity_shards: usize) -> Self {
        let rs = ReedSolomon::new(data_shards, parity_shards)
            .expect("invalid RS parameters");
        Self {
            rs,
            data_shards,
            parity_shards,
        }
    }

    pub fn encode(&self, shards: &mut Vec<Vec<u8>>) {
        self.rs.encode(shards).expect("RS encode failed");
    }

    pub fn reconstruct(&self, shards: &mut Vec<Option<Vec<u8>>>) -> bool {
        self.rs.reconstruct(shards).is_ok()
    }
}
