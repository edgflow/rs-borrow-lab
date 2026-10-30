//! Part 3: returning references: which input does the result borrow from?

fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn longer(a: &str, b: &str) -> &str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let title = String::from("lifetimes in signatures");
    println!("first word: {}", first_word(&title));
    println!("longer: {}", longer(&title, "part three"));
}
