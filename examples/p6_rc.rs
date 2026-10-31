//! Part 6: MyRc<T>: one value, many owners, a count on the heap.

use borrow_lab::Noisy;
use borrow_lab::my_rc::MyRc;

fn main() {
    let a = MyRc::new(Noisy("shared"));
    let b = a.clone(); // a second owner: the count goes up, the Noisy is not copied
    println!("count after clone: {}", MyRc::strong_count(&a));
    drop(a);
    println!("count after drop: {}", MyRc::strong_count(&b));
    println!("still here: {}", b.0);
}
