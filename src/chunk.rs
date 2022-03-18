use std::ops::Index;
use std::ptr;

use crate::memory::{free_array, grow_array, grow_capacity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    Return,
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(op: u8) -> Result<Self, Self::Error> {
        if op >= (OpCode::Return as u8) && op < ((OpCode::Return as u8) + 1) {
            // We know that it's a valid Opcode here so we can transmute
            Ok(unsafe { std::mem::transmute(op) })
        } else {
            Err(())
        }
    }
}

pub struct Chunk {
    code: *mut u8,
    pub(crate) count: usize,
    capacity: usize,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            count: 0,
            capacity: 0,
            code: ptr::null_mut(),
        }
    }

    pub fn write(&mut self, byte: u8) {
        if self.capacity < self.count + 1 {
            let old_capacity = self.capacity;
            self.capacity = grow_capacity(old_capacity);
            self.code = grow_array(self.code, old_capacity, self.capacity)
        }

        unsafe {
            ptr::write(self.code.offset(self.count as isize), byte);
        }
        self.count += 1;
    }
}

impl Index<usize> for Chunk {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.code.offset(index as isize).as_ref().unwrap() }
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        free_array(self.code, self.capacity);
        // We don't zero the fields because Rust won't let us use this
        // after it's freed
    }
}
