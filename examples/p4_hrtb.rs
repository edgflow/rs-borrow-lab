//! Part 4: higher-ranked lifetimes: a closure that must work for every lifetime.

/// Calls `f` on a String that only lives inside `measure`.
fn measure<'a, F>(f: F) -> usize
where
    F: Fn(&'a str) -> usize,
{
    let local = String::from("made inside measure");
    f(&local)
}

fn main() {
    println!("{}", measure(|s| s.len()));
}
