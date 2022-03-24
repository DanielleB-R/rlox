use std::process;

use rlox::chunk::{Chunk, OpCode};
use rlox::debug::disassemble_chunk;

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::Constant.into());
    chunk.write(constant as u8);

    chunk.write(OpCode::Return.into());
    disassemble_chunk(&chunk, "test chunk");
    drop(chunk);
    process::exit(0);
}
