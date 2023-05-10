use std::ptr;
use std::{fmt::Display, ops::Index};

use crate::memory::{free_array, grow_array, grow_capacity};
use crate::object::{Obj, ObjString};

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    Obj(*mut Obj),
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Self::Number(n)
    }
}

impl From<*mut Obj> for Value {
    fn from(value: *mut Obj) -> Self {
        Self::Obj(value)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Number(n) => write!(f, "{}", n),
            Self::Obj(o) => write!(f, "\"{}\"", unsafe { &**o }),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::Obj(a), Self::Obj(b)) => unsafe { **a == **b },
            _ => false,
        }
    }
}

impl Value {
    pub fn is_number(&self) -> bool {
        if let Self::Number(_) = self {
            true
        } else {
            false
        }
    }

    pub fn as_number(&self) -> f64 {
        if let Self::Number(n) = self {
            *n
        } else {
            panic!("not a number");
        }
    }

    pub fn is_string(&self) -> bool {
        if let Self::Obj(o) = self {
            match unsafe { &**o } {
                Obj::String(_) => true,
                // _ => false,
            }
        } else {
            false
        }
    }

    pub fn as_string(&self) -> &ObjString {
        if let Self::Obj(o) = self {
            if let Obj::String(s) = unsafe { &**o } {
                s
            } else {
                panic!("not a string");
            }
        } else {
            panic!("not a string");
        }
    }
}

pub struct ValueArray {
    capacity: usize,
    pub(crate) count: usize,
    values: *mut Value,
}

impl ValueArray {
    pub fn new() -> Self {
        Self {
            count: 0,
            capacity: 0,
            values: ptr::null_mut(),
        }
    }

    pub fn write(&mut self, value: Value) {
        if self.capacity < self.count + 1 {
            let old_capacity = self.capacity;
            self.capacity = grow_capacity(old_capacity);
            self.values = grow_array(self.values, old_capacity, self.capacity)
        }

        unsafe {
            ptr::write(self.values.add(self.count), value);
        }
        self.count += 1;
    }
}

impl Index<usize> for ValueArray {
    type Output = Value;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.values.add(index) }
    }
}

impl Drop for ValueArray {
    fn drop(&mut self) {
        free_array(self.values, self.capacity);
        // We don't zero the fields because Rust won't let us use this
        // after it's freed
    }
}
