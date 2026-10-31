//! MyBox<T>: a Box built by hand.

use std::alloc::{Layout, alloc, handle_alloc_error};
use std::ops::Deref;
use std::ptr::NonNull;

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
