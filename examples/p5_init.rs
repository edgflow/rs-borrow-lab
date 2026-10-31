//! Part 5: partial initialization: not in safe Rust; field by field with MaybeUninit.

use std::mem::MaybeUninit;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut slot = MaybeUninit::<Point>::uninit(); // room for a Point, nothing in it yet
    let ptr = slot.as_mut_ptr();
    unsafe {
        (&raw mut (*ptr).x).write(1); // write each field through a raw pointer
        (&raw mut (*ptr).y).write(2);
    }
    let p = unsafe { slot.assume_init() }; // our promise: every field is written
    println!("{p:?}");
}
