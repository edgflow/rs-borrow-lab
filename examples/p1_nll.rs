//! Part 1: a borrow lives until its last use, not until the end of its scope.

fn main() {
    let mut scores = vec![3, 1, 4, 1, 5];

    let best = scores.iter().max().unwrap(); // borrows scores
    println!("best = {best}"); // the last use of `best`
    scores.push(9); // fine: the borrow is over

    let first = &scores[0];
    if *first > 2 {
        println!("first = {first}"); // the last use, on this path only
    }
    scores.retain(|&s| s != 1); // fine on both paths

    for s in &scores {
        if *s > 3 {
            scores.push(s * 10); // the loop still borrows scores
        }
    }
}
