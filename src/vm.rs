use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

use std::ptr;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    chunk: *const Chunk,
    ip: *const u8,
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunk: ptr::null(),
            ip: ptr::null(),
        }
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        self.chunk = chunk as *const Chunk;
        self.ip = chunk.code as *const u8;
        self.run()
    }

    #[inline]
    unsafe fn read_byte(&mut self) -> u8 {
        let value = *self.ip;
        self.ip = self.ip.offset(1);
        value
    }

    #[inline]
    unsafe fn read_constant(&mut self) -> Value {
        (*(self.chunk)).constants[self.read_byte().into()]
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = unsafe { OpCode::from_byte(self.read_byte()) };

            match instruction {
                OpCode::Constant => {
                    let constant = unsafe { self.read_constant() };
                    println!("{}", constant);
                }
                OpCode::Return => return InterpretResult::Ok,
                _ => unimplemented!(),
            }
        }
    }
}
