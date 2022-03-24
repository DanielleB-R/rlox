use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.count {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    let instruction = chunk[offset];
    match OpCode::try_from(instruction) {
        Ok(OpCode::Constant) => constant_instruction("OP_CONSTANT", chunk, offset),
        Ok(OpCode::Return) => simple_instruction("OP_RETURN", offset),
        Err(()) => {
            println!("Unknown opcode {}", instruction);
            offset + 1
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk[offset + 1];
    println!(
        "{:16} {:4} '{}'",
        name, constant, chunk.constants[constant as usize]
    );
    offset + 2
}
