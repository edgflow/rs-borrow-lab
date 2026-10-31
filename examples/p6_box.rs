//! Part 6: MyBox<T>, a Box built by hand.

use borrow_lab::Noisy;
use borrow_lab::my_box::MyBox;

fn greet(name: &str) {
    println!("hello, {name}");
}

fn main() {
    let b = MyBox::new(Noisy("boxed"));
    println!("inside: {}", b.0); // b.0 is (*b).0: through Deref

    let mut name = MyBox::new(String::from("ferris"));
    name.push_str("!"); // a &mut String, through DerefMut
    greet(&name); // &MyBox<String> → &String → &str: deref coercion
    println!("end of main");
}
