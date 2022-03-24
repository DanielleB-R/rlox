use std::ops::Index;
use std::ptr;

use crate::memory::{free_array, grow_array, grow_capacity};

pub type Value = f64;

// add impl Display here later

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
            ptr::write(self.values.offset(self.count as isize), value);
        }
        self.count += 1;
    }
}

impl Index<usize> for ValueArray {
    type Output = Value;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.values.offset(index as isize) }
    }
}

impl Drop for ValueArray {
    fn drop(&mut self) {
        free_array(self.values, self.capacity);
        // We don't zero the fields because Rust won't let us use this
        // after it's freed
    }
}
