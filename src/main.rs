mod crypto;
mod protocol;
mod transport;
mod fec;

use protocol::chunk::ChunkReader;

fn main() {
    let mut reader =
        ChunkReader::new("testfile.bin").expect("file open failed");

    while let Some((seq, chunk)) =
        reader.next_chunk().expect("read failed")
    {
        println!(
            "Chunk {} | size {} bytes",
            seq,
            chunk.len()
        );
    }
}
