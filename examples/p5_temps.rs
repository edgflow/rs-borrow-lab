//! Part 5: temporaries: values without a name, and how long they live.

use borrow_lab::Noisy;

fn main() {
    println!("length {}", Noisy("temp").0.len()); // the temporary dies at the ;
    println!("after the statement");

    let extended = &Noisy("extended"); // & in a let: the temporary lives to the end of the block
    let text = &String::from("lifetimes"); // the same: a &String that stays valid
    println!("{} {text}", extended.0);
    println!("end of main");
}
