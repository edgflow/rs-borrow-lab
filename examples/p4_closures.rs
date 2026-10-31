//! Part 4: closures that return a reference, and Box<dyn Fn(&str) -> &str>.

fn main() {
    let trim = |s: &str| s.trim(); // returns a slice of its argument
    println!("[{}]", trim("  # title  "));
}
