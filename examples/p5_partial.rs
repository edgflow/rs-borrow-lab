//! Part 5: partial moves, and why a type with Drop can't be taken apart.

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

/// A type with a destructor: its drop needs all of its fields.
struct Ticket {
    id: String,
}

impl Drop for Ticket {
    fn drop(&mut self) {
        println!("  drop ticket {:?}", self.id);
    }
}

fn main() {
    let ana = Person {
        name: String::from("ana"),
        age: 31,
    };
    let name = ana.name; // moves one field out
    println!("{name} is {}", ana.age); // the other field is still usable

    let mut ticket = Ticket {
        id: String::from("T-1"),
    };
    let id = std::mem::take(&mut ticket.id); // take it, and leave an empty String behind
    println!("took {id}");
}
