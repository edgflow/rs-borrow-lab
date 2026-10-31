//! Part 7: a struct that points into itself: a move breaks it, and Pin forbids the move.

use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr;

/// A name, and a raw pointer to that name: to a field of the same struct.
struct SelfRef {
    name: String,
    to_name: *const String,
    _pinned: PhantomPinned, // SelfRef is not Unpin: once pinned, it stays where it is
}

impl SelfRef {
    fn new(name: &str) -> Pin<Box<SelfRef>> {
        let mut boxed = Box::pin(SelfRef {
            name: name.to_string(),
            to_name: ptr::null(),
            _pinned: PhantomPinned,
        });
        let to_name: *const String = &boxed.name; // its final address: on the heap
        unsafe { boxed.as_mut().get_unchecked_mut().to_name = to_name }; // no move
        boxed
    }

    fn points_at_itself(&self) -> bool {
        ptr::eq(self.to_name, &self.name)
    }
}

fn main() {
    let a = SelfRef::new("ferris");
    println!("pinned: {}", a.points_at_itself());
    let mut b = a; // moves the Box, a pointer: the SelfRef stays where it is
    println!("after moving the box: {}", b.points_at_itself());
    let mut c = SelfRef::new("crab");
    std::mem::swap(&mut b, &mut c); // swaps the two boxes: pointers, not the SelfRefs
    let (b_ok, c_ok) = (b.points_at_itself(), c.points_at_itself());
    println!("after swapping the boxes: {b_ok} {c_ok}");
}
