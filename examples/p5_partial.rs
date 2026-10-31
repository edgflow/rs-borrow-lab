//! Part 5: partial moves, and why a type with Drop can't be taken apart.

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let ana = Person {
        name: String::from("ana"),
        age: 31,
    };
    let name = ana.name; // moves one field out
    println!("{name} is {}", ana.age); // the other field is still usable
    println!("{ana:?}"); // but the whole struct is not
}
