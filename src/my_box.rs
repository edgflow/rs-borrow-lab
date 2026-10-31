//! MyBox<T>: a Box built by hand.

use std::alloc::{Layout, alloc, dealloc, handle_alloc_error};
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

/// Owns one T on the heap, like Box<T>.
pub struct MyBox<T> {
    ptr: NonNull<T>,
}

impl<T> MyBox<T> {
    pub fn new(value: T) -> MyBox<T> {
        let layout = Layout::new::<T>(); // the size and alignment of a T
        let raw = unsafe { alloc(layout) } as *mut T;
        let ptr = NonNull::new(raw).unwrap_or_else(|| handle_alloc_error(layout));
        unsafe { ptr.as_ptr().write(value) }; // move the value into the heap
        MyBox { ptr }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref() } // a shared reference to the T on the heap
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() } // unique: we own the T, and self is borrowed mutably
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(self.ptr.as_ptr()); // drop the T first
            dealloc(self.ptr.as_ptr() as *mut u8, Layout::new::<T>()); // then the memory
        }
    }
}
