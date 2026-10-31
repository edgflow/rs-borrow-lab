//! Part 6: drop check: a Drop impl may use what its T borrows.

use borrow_lab::my_box::MyBox;

fn main() {
    let (std_box, text); // declared together: text is dropped first
    text = String::from("hello");
    std_box = Box::new(&text); // std's Box: accepted
    println!("{std_box}");

    let text2 = String::from("hello"); // declared first, dropped last
    let my_box = MyBox::new(&text2); // dropped before text2: its Drop may read it
    println!("{}", *my_box);
}
