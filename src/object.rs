use crate::memory;
use std::fmt::Display;
use std::ptr::copy_nonoverlapping;
use std::{slice, str};

#[derive(Debug, Clone, PartialEq)]
pub enum Obj {
    String(ObjString),
}

impl Obj {
    pub fn copy_string(string: &str) -> *mut Self {
        ObjString::from_str(string).into()
    }

    pub fn take_string(chars: *mut u8, length: usize) -> *mut Self {
        ObjString::new(chars, length).into()
    }
}

impl Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(string) => write!(f, "{}", string.as_ruststr()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObjString {
    pub length: usize,
    pub chars: *mut u8,
}

impl ObjString {
    fn as_ruststr(&self) -> &str {
        unsafe {
            let slice = slice::from_raw_parts_mut(self.chars, self.length);
            str::from_utf8_unchecked(slice)
        }
    }

    fn from_str(slice: &str) -> Self {
        let length = slice.len() - 2;
        let heap_chars = memory::allocate(length);
        unsafe {
            copy_nonoverlapping(slice.as_ptr().add(1), heap_chars, length);
        }
        Self {
            length,
            chars: heap_chars,
        }
    }

    fn new(chars: *mut u8, length: usize) -> Self {
        Self { chars, length }
    }
}

impl PartialEq for ObjString {
    fn eq(&self, other: &Self) -> bool {
        self.as_ruststr() == other.as_ruststr()
    }
}

impl Into<*mut Obj> for ObjString {
    fn into(self) -> *mut Obj {
        Box::into_raw(Box::new(Obj::String(self)))
    }
}
