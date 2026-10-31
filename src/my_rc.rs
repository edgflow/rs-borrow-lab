//! MyRc<T> and MyWeak<T>: shared ownership with reference counts, like Rc<T> and Weak<T>.

use std::alloc::{Layout, dealloc};
use std::cell::Cell;
use std::ops::Deref;
use std::ptr::{self, NonNull};

/// What lives on the heap: the two counts, next to the value.
struct RcInner<T> {
    strong: Cell<usize>,
    weak: Cell<usize>,
    value: T,
}

/// One owner of a shared T; cloning it adds an owner, not a copy.
pub struct MyRc<T> {
    ptr: NonNull<RcInner<T>>,
}

/// Points at a shared T without owning it: the T can be gone.
pub struct MyWeak<T> {
    ptr: NonNull<RcInner<T>>,
}

impl<T> MyRc<T> {
    pub fn new(value: T) -> MyRc<T> {
        let inner = Box::new(RcInner {
            strong: Cell::new(1),
            weak: Cell::new(1), // one weak, held by all the strong owners together
            value,
        });
        MyRc {
            ptr: NonNull::from(Box::leak(inner)), // Box allocates, hands us the pointer
        }
    }

    pub fn strong_count(this: &MyRc<T>) -> usize {
        this.inner().strong.get()
    }

    pub fn downgrade(this: &MyRc<T>) -> MyWeak<T> {
        let weak = &this.inner().weak;
        weak.set(weak.get() + 1);
        MyWeak { ptr: this.ptr }
    }

    fn inner(&self) -> &RcInner<T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> MyWeak<T> {
    pub fn upgrade(&self) -> Option<MyRc<T>> {
        let strong = self.strong();
        if strong.get() == 0 {
            return None; // the value is already gone
        }
        strong.set(strong.get() + 1);
        Some(MyRc { ptr: self.ptr })
    }

    // only the counts, never the whole RcInner: the value may be in the middle of its drop
    fn strong(&self) -> &Cell<usize> {
        unsafe { &(*self.ptr.as_ptr()).strong }
    }

    fn weak(&self) -> &Cell<usize> {
        unsafe { &(*self.ptr.as_ptr()).weak }
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
        if strong.get() > 0 {
            return;
        }
        unsafe { ptr::drop_in_place(&raw mut (*self.ptr.as_ptr()).value) }; // the value
        drop(MyWeak { ptr: self.ptr }); // then lets go of the weak the owners shared
    }
}

impl<T> Drop for MyWeak<T> {
    fn drop(&mut self) {
        let weak = self.weak();
        weak.set(weak.get() - 1);
        if weak.get() == 0 {
            let layout = Layout::new::<RcInner<T>>();
            unsafe { dealloc(self.ptr.as_ptr() as *mut u8, layout) }; // nobody points here
        }
    }
}
