//! Part 1: many shared borrows, or one mutable borrow, never both at once.

fn main() {
    let mut v = vec![10];
    let first = &v[0]; // a shared borrow of v
    println!("first = {first}"); // its last use: the borrow ends here
    v.push(20); // so the mutable borrow is fine now
    println!("v = {v:?}");

    // why the rule exists: a push may move every element to a new, bigger buffer
    let mut grow: Vec<u32> = Vec::new();
    let mut at = grow.as_ptr();
    for i in 0..40 {
        grow.push(i);
        if grow.as_ptr() != at {
            at = grow.as_ptr();
            let (len, cap) = (grow.len(), grow.capacity());
            println!("len {len:>2}, capacity {cap:>2}: the buffer moved to {at:p}");
        }
    }
}
