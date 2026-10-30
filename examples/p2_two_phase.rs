//! Part 2: two-phase borrows: why `v.push(v.len())` compiles.

fn main() {
    let mut v: Vec<usize> = vec![10, 20];
    v.push(v.len()); // &mut v is only reserved here, and activated at the call
    println!("{v:?}");

    let n = v.len(); // read first
    let m = &mut v; // then borrow mutably
    m.push(n);
    v.extend_from_within(..); // copy from itself, inside one &mut borrow
    println!("{v:?}");
}
