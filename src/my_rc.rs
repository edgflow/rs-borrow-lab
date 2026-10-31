//! MyRc<T>: shared ownership with a reference count, like Rc<T>.

use std::cell::Cell;
use std::ops::Deref;
use std::ptr::NonNull;

/// What lives on the heap: the count, next to the value.
struct RcInner<T> {
    strong: Cell<usize>,
    value: T,
}

/// One owner of a shared T; cloning it adds an owner, not a copy.
pub struct MyRc<T> {
    ptr: NonNull<RcInner<T>>,
}

impl<T> MyRc<T> {
    pub fn new(value: T) -> MyRc<T> {
        let inner = Box::new(RcInner {
            strong: Cell::new(1),
            value,
        });
        MyRc {
            ptr: NonNull::from(Box::leak(inner)), // Box allocates, hands us the pointer
        }
    }

    pub fn strong_count(this: &MyRc<T>) -> usize {
        this.inner().strong.get()
    }

    fn inner(&self) -> &RcInner<T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> MyRc<T> {
        let strong = &self.inner().strong;
        strong.set(strong.get() + 1); // one more owner, same allocation
        MyRc { ptr: self.ptr }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner().value
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        let strong = &self.inner().strong;
        strong.set(strong.get() - 1);
        if strong.get() == 0 {
            unsafe { drop(Box::from_raw(self.ptr.as_ptr())) }; // the last owner frees it
        }
    }
}
