//! Part 5: temporaries: values without a name, and how long they live.

use borrow_lab::Noisy;

fn main() {
    println!("length {}", Noisy("temp").0.len()); // the temporary dies at the ;
    println!("after the statement");

    let text: &str = String::from("lifetimes").as_str(); // a temporary String
    println!("{text}");
}
