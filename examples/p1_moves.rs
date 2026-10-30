//! Part 1: a move copies the owner, never the heap data.

use std::mem::size_of;

fn main() {
    let a = String::from("borrow checker");
    println!("a owns {} bytes on the heap at {:p}", a.len(), a.as_ptr());

    let b = a; // move: `b` is the owner now
    println!("b owns the same bytes at {:p}", b.as_ptr());
    println!("what moved: {} bytes", size_of::<String>()); // pointer, length, capacity

    let c = b.clone(); // clone: a second buffer, and a second owner
    println!("c owns a copy at {:p}", c.as_ptr());

    let x = 42; // i32 is Copy: copying it copies all of it
    let y = x;
    println!("x = {x}, y = {y}: both still usable");
}
