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
    if offset > 0 && chunk.line(offset) == chunk.line(offset - 1) {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.line(offset));
    }

    let instruction = chunk[offset];
    match OpCode::try_from(instruction) {
        Ok(OpCode::Constant) => constant_instruction("OP_CONSTANT", chunk, offset),
        Ok(OpCode::Nil) => simple_instruction("OP_NIL", offset),
        Ok(OpCode::True) => simple_instruction("OP_TRUE", offset),
        Ok(OpCode::False) => simple_instruction("OP_FALSE", offset),
        Ok(OpCode::Equal) => simple_instruction("OP_EQUAL", offset),
        Ok(OpCode::Greater) => simple_instruction("OP_GREATER", offset),
        Ok(OpCode::Less) => simple_instruction("OP_FALSE", offset),
        Ok(OpCode::Add) => simple_instruction("OP_ADD", offset),
        Ok(OpCode::Subtract) => simple_instruction("OP_SUBTRACT", offset),
        Ok(OpCode::Multiply) => simple_instruction("OP_MULTIPLY", offset),
        Ok(OpCode::Divide) => simple_instruction("OP_DIVIDE", offset),
        Ok(OpCode::Not) => simple_instruction("OP_NOT", offset),
        Ok(OpCode::Negate) => simple_instruction("OP_NEGATE", offset),
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
