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
        Box::into_raw(Box::new(Obj::String(ObjString::from_str(string))))
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
    length: usize,
    chars: *mut u8,
}

impl ObjString {
    fn as_ruststr(&self) -> &str {
        unsafe {
            let slice = slice::from_raw_parts_mut(self.chars, self.length);
            str::from_utf8_unchecked(slice)
        }
    }

    fn from_str(slice: &str) -> Self {
        let length = slice.len();
        let heap_chars = memory::allocate(slice.len());
        unsafe {
            copy_nonoverlapping(slice.as_ptr(), heap_chars, slice.len());
        }
        Self {
            length,
            chars: heap_chars,
        }
    }
}

impl PartialEq for ObjString {
    fn eq(&self, other: &Self) -> bool {
        self.as_ruststr() == other.as_ruststr()
    }
}
