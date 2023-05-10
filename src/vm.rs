use crate::chunk::{Chunk, OpCode};
use crate::compiler::compile;
#[cfg(debug_assertions)]
use crate::debug::disassemble_instruction;
use crate::memory::allocate;
use crate::object::Obj;
use crate::value::Value;

use std::ptr::{self, copy_nonoverlapping};

const MAX_STACK: usize = 256;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

macro_rules! binary_op {
    ($stack:ident, $op:tt) => {
        {
            if $stack.peek(0).is_number() && $stack.peek(1).is_number() {
                let b = $stack.pop().as_number();
                let a = $stack.pop().as_number();
                $stack.push((a $op b).into());
            } else {
                $stack.runtime_error("Operands must be numbers.");
                return InterpretResult::RuntimeError;
            }
        }
    }
}

fn is_falsey(value: Value) -> bool {
    value == Value::Nil || value == Value::Bool(false)
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
            stack: [0.0.into(); MAX_STACK],
            stack_top: ptr::null_mut(),
        };
        value.reset_stack();
        value
    }

    fn runtime_error(&mut self, message: &str) {
        eprintln!("{}", message);

        let instruction = unsafe { self.ip.offset_from((*self.chunk).code) - 1 };
        let line = unsafe { *((*self.chunk).lines.offset(instruction)) };
        eprintln!("[line {}] in script", line);
        self.reset_stack();
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

    fn peek(&self, distance: usize) -> &Value {
        unsafe { self.stack_top.sub(1 + distance).as_ref().unwrap() }
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

    fn concatenate(&mut self) {
        let b = self.pop();
        let a = self.pop();

        let b_str = b.as_string();
        let a_str = a.as_string();

        let length = a_str.length + b_str.length;
        let chars = allocate(length);
        unsafe {
            copy_nonoverlapping(a_str.chars, chars, a_str.length);
            copy_nonoverlapping(b_str.chars, chars.add(a_str.length), b_str.length);
        }

        self.push(Obj::take_string(chars, length).into())
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
                OpCode::Nil => self.push(Value::Nil),
                OpCode::True => self.push(Value::Bool(true)),
                OpCode::False => self.push(Value::Bool(false)),
                OpCode::Equal => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push((a == b).into());
                }
                OpCode::Greater => binary_op!(self, >),
                OpCode::Less => binary_op!(self, <),
                OpCode::Add => {
                    if self.peek(0).is_string() && self.peek(1).is_string() {
                        self.concatenate();
                    } else {
                        if self.peek(0).is_number() && self.peek(1).is_number() {
                            let b = self.pop().as_number();
                            let a = self.pop().as_number();
                            self.push((a + b).into());
                        } else {
                            self.runtime_error("Operands must both be numbers or strings.");
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::Subtract => binary_op!(self, -),
                OpCode::Multiply => binary_op!(self, *),
                OpCode::Divide => binary_op!(self, /),
                OpCode::Not => {
                    let value = self.pop();
                    self.push(is_falsey(value).into())
                }
                OpCode::Negate => {
                    if self.peek(0).is_number() {
                        let n = self.pop().as_number();
                        self.push((-n).into());
                    } else {
                        self.runtime_error("Operand must be a number");
                        return InterpretResult::RuntimeError;
                    }
                }
                OpCode::Return => {
                    println!("{}", self.pop());
                    return InterpretResult::Ok;
                }
                _ => unimplemented!(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stack() {
        let mut vm = VM::new();

        vm.push(Value::Nil);
        assert_eq!(vm.peek(0), &Value::Nil);
        assert_eq!(vm.pop(), Value::Nil);

        vm.push(Value::Bool(true));
        vm.push(Value::Number(25.0));
        assert_eq!(vm.peek(1), &Value::Bool(true));
        assert_eq!(vm.pop(), Value::Number(25.0));
        assert_eq!(vm.pop(), Value::Bool(true));
    }
}
