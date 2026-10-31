//! Part 4: subtyping and variance: when a longer lifetime can stand in for a shorter one.

/// Returns the first word; it lives as long as the words in the list.
fn first<'a>(words: &[&'a str]) -> &'a str {
    words[0]
}

fn main() {
    let local = String::from("local");
    let fixed: Vec<&'static str> = vec!["static", "words"];
    println!("{}", first(&fixed)); // here 'a is 'static

    let mut mixed: Vec<&str> = fixed; // a Vec<&'static str> used as a Vec of shorter &str
    mixed.push(&local); // so it can hold a shorter-lived word too
    println!("{mixed:?} {}", first(&mixed));
}
