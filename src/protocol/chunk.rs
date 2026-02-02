use std::fs::File;
use std::io::{self, Read};
use crate::protocol::constants::CHUNK_SIZE;

pub struct ChunkReader {
    file: File,
    seq_no: u32,
}

impl ChunkReader {
    pub fn new(path: &str) -> io::Result<Self> {
        Ok(Self {
            file: File::open(path)?,
            seq_no: 0,
        })
    }

    pub fn next_chunk(&mut self) -> io::Result<Option<(u32, Vec<u8>)>> {
        let mut buf = vec![0u8; CHUNK_SIZE];
        let bytes_read = self.file.read(&mut buf)?;

        if bytes_read == 0 {
            return Ok(None);
        }

        buf.truncate(bytes_read);
        let seq = self.seq_no;
        self.seq_no += 1;

        Ok(Some((seq, buf)))
    }
}
