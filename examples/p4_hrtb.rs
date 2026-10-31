//! Part 4: higher-ranked lifetimes: a closure that must work for every lifetime.

/// Calls `f` on a String that only lives inside `measure`.
fn measure<F>(f: F) -> usize
where
    F: for<'a> Fn(&'a str) -> usize, // for every lifetime: the same as Fn(&str) -> usize
{
    let local = String::from("made inside measure");
    f(&local)
}

fn main() {
    println!("{}", measure(|s| s.len()));
    println!("{}", measure(|s| s.split(' ').count()));

    let any = |s: &str| s.len(); // takes a &str of any lifetime
    println!("{}", measure(any));
}
