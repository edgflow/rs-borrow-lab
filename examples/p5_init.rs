//! Part 5: partial initialization: not in safe Rust; field by field with MaybeUninit.

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p: Point;
    p.x = 1; // assign one field of a struct that doesn't exist yet
    p.y = 2;
    println!("{p:?}");
}
