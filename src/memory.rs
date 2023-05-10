use std::{
    alloc::{alloc, dealloc, realloc, Layout},
    mem, ptr,
};

#[inline]
pub fn grow_capacity(capacity: usize) -> usize {
    if capacity < 8 {
        8
    } else {
        capacity * 2
    }
}

#[inline]
pub fn allocate<T>(count: usize) -> *mut T {
    reallocate(ptr::null_mut(), 0, count * mem::size_of::<T>()) as *mut T
}

pub fn free_array<T>(pointer: *mut T, old_size: usize) {
    reallocate(pointer as *mut u8, old_size * mem::size_of::<T>(), 0);
}

pub fn grow_array<T>(pointer: *mut T, old_size: usize, new_size: usize) -> *mut T {
    let type_size = mem::size_of::<T>();
    return reallocate(
        pointer as *mut u8,
        old_size * type_size,
        new_size * type_size,
    ) as *mut T;
}

pub fn reallocate(pointer: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
    if new_size == 0 {
        unsafe {
            dealloc(pointer, Layout::from_size_align(old_size, 8).unwrap());
        }
        return ptr::null_mut();
    }

    if old_size == 0 {
        return unsafe { alloc(Layout::from_size_align(new_size, 8).unwrap()) };
    }

    let new_ptr = unsafe {
        realloc(
            pointer,
            Layout::from_size_align(old_size, 8).unwrap(),
            new_size,
        )
    };
    if new_ptr == ptr::null_mut() {
        pointer
    } else {
        new_ptr
    }
}
