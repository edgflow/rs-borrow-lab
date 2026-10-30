//! Part 1: one owner on the heap, or many owners of one value.

use borrow_lab::Noisy;
use std::mem::size_of_val;
use std::rc::Rc;

fn main() {
    let boxed = Box::new(Noisy("boxed")); // one owner, the value lives on the heap
    let moved = boxed; // moving a Box moves the pointer, not the value
    println!("a Box on the stack: {} bytes", size_of_val(&moved));

    let first = Rc::new(Noisy("shared")); // shared ownership: a count of owners
    println!("owners: {}", Rc::strong_count(&first));
    let second = Rc::clone(&first); // a second owner of the same value
    println!("owners: {}", Rc::strong_count(&first));
    drop(first);
    println!(
        "owners: {}, the value is still alive",
        Rc::strong_count(&second)
    );
    drop(second); // the last owner is gone: now it is dropped
    println!("main ends");
}
