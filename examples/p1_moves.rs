//! Part 1: a move copies the owner, never the heap data.

fn main() {
    let a = String::from("borrow checker");
    println!("a owns {} bytes on the heap at {:p}", a.len(), a.as_ptr());

    let b = a; // move: `b` is the owner now
    println!("b owns the same bytes at {:p}", b.as_ptr());
    println!("a is still {a}");
}
