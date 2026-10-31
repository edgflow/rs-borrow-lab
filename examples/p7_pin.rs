//! Part 7: a struct that points into itself: a move breaks it, and Pin forbids the move.

use std::ptr;

/// A name, and a raw pointer to that name: to a field of the same struct.
struct SelfRef {
    name: String,
    to_name: *const String,
}

impl SelfRef {
    fn new(name: &str) -> SelfRef {
        SelfRef {
            name: name.to_string(),
            to_name: ptr::null(),
        }
    }

    fn init(&mut self) {
        self.to_name = &self.name; // where name is right now
    }

    fn points_at_itself(&self) -> bool {
        ptr::eq(self.to_name, &self.name)
    }
}

fn main() {
    let mut a = SelfRef::new("ferris");
    a.init();
    println!("before the move: {}", a.points_at_itself());
    let b = a; // the bytes move to b; to_name still holds a's old address
    println!("after the move: {}", b.points_at_itself());
}
