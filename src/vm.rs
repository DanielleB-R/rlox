use crate::chunk::{Chunk, OpCode};
use crate::compiler::compile;
#[cfg(debug_assertions)]
use crate::debug::disassemble_instruction;
use crate::value::Value;

use std::ptr;

const MAX_STACK: usize = 256;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

macro_rules! binary_op {
    ($stack:ident, $op:tt) => {
        {
            let b = $stack.pop();
            let a = $stack.pop();
            $stack.push(a $op b);
        }
    }
}

pub struct VM {
    chunk: *const Chunk,
    ip: *const u8,
    stack: [Value; MAX_STACK],
    stack_top: *mut Value,
}

impl VM {
    pub fn new() -> Self {
        let mut value = Self {
            chunk: ptr::null(),
            ip: ptr::null(),
            stack: [0.0; MAX_STACK],
            stack_top: ptr::null_mut(),
        };
        value.reset_stack();
        value
    }

    fn reset_stack(&mut self) {
        self.stack_top = self.stack.as_mut_ptr();
    }

    fn push(&mut self, value: Value) {
        unsafe {
            self.stack_top.write(value);
            self.stack_top = self.stack_top.add(1);
        }
    }

    fn pop(&mut self) -> Value {
        unsafe {
            self.stack_top = self.stack_top.sub(1);
            *self.stack_top
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut chunk = Chunk::new();

        if !compile(source, &mut chunk) {
            return InterpretResult::CompileError;
        }

        self.chunk = &chunk as *const Chunk;
        self.ip = chunk.code as *const u8;

        let result = self.run();

        self.chunk = ptr::null();
        self.ip = ptr::null();

        result
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
            #[cfg(debug_assertions)]
            unsafe {
                print!("          ");
                let mut slot = self.stack.as_mut_ptr();
                while slot != self.stack_top {
                    print!("[ {} ]", *slot);
                    slot = slot.add(1);
                }
                println!("");
                disassemble_instruction(
                    &*self.chunk,
                    self.ip.offset_from((*self.chunk).code) as usize,
                );
            }

            let instruction = unsafe { OpCode::from_byte(self.read_byte()) };

            match instruction {
                OpCode::Constant => {
                    let constant = unsafe { self.read_constant() };
                    self.push(constant);
                }
                OpCode::Negate => {
                    let value = self.pop();
                    self.push(-value);
                }
                OpCode::Add => binary_op!(self, +),
                OpCode::Subtract => binary_op!(self, -),
                OpCode::Multiply => binary_op!(self, *),
                OpCode::Divide => binary_op!(self, /),
                OpCode::Return => {
                    println!("{}", self.pop());
                    return InterpretResult::Ok;
                }
                _ => unimplemented!(),
            }
        }
    }
}
