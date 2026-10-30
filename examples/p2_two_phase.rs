//! Part 2: two-phase borrows: why `v.push(v.len())` compiles.

fn main() {
    let mut v: Vec<usize> = vec![10, 20];
    v.push(v.len()); // &mut v is only reserved here, and activated at the call
    println!("{v:?}");

    let m = &mut v; // an explicit &mut is active at once
    m.push(v.len()); // so reading v here conflicts with it
    v.extend(v.iter()); // and here the shared borrow is still in use inside extend
}
