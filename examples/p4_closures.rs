//! Part 4: closures that return a reference, and Box<dyn Fn(&str) -> &str>.

fn main() {
    // the type says: for every lifetime 'a, a &'a str in, a &'a str out
    let trim: Box<dyn Fn(&str) -> &str> = Box::new(|s| s.trim());
    println!("[{}]", trim("  # title  "));

    let steps: Vec<Box<dyn Fn(&str) -> &str>> = vec![
        Box::new(|s| s.trim()),
        Box::new(|s| s.trim_start_matches('#')),
        Box::new(|s| s.trim()),
    ];
    let mut text = "  # lifetimes  ";
    for step in &steps {
        text = step(text); // each step returns a slice of the same text
    }
    println!("[{text}]");
}
