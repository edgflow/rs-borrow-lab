//! Part 3: returning references: which input does the result borrow from?

fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let title = String::from("lifetimes in signatures");
    println!("first word: {}", first_word(&title));

    let result;
    {
        let subtitle = String::from("advanced ownership, part three");
        result = longer(&title, &subtitle);
    } // subtitle is dropped here
    println!("longer: {result}");
}
