//! Part 6: MyBox<T>, a Box built by hand.

use borrow_lab::Noisy;
use borrow_lab::my_box::MyBox;

fn main() {
    let b = MyBox::new(Noisy("boxed"));
    println!("inside: {}", b.0); // b.0 is (*b).0: through Deref
    println!("end of main");
}
