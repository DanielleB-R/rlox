use std::process;

use rlox::chunk::{Chunk, OpCode};
use rlox::debug::disassemble_chunk;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::Return as u8);
    disassemble_chunk(&chunk, "test chunk");
    drop(chunk);
    process::exit(0);
}
