//! Part 2: two mutable references into one vector.

fn main() {
    let mut v = vec![1, 2, 3, 4];
    let a = &mut v[0]; // a mutable borrow of v
    let b = &mut v[3]; // a second one, while `a` is still in use
    *a += 10;
    *b += 10;
    println!("{v:?}");
}
