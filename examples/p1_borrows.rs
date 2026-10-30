//! Part 1: many shared borrows, or one mutable borrow, never both at once.

fn main() {
    let mut v = vec![10];
    let first = &v[0]; // a shared borrow of v
    v.push(20); // push needs `&mut v` while `first` is still in use
    println!("first = {first}");
}
