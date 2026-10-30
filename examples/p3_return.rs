//! Part 3: returning references: which input does the result borrow from?

fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn greeting(name: &str) -> String {
    format!("hello, {name}") // the caller owns the result
}

fn default_name() -> &'static str {
    "guest" // a literal lives in the program itself
}

/// The result borrows from `a` only, so `b` may die first.
fn first_of<'a, 'b>(a: &'a str, _b: &'b str) -> &'a str {
    a
}

fn main() {
    let title = String::from("lifetimes in signatures");
    println!("first word: {}", first_word(&title));

    let result;
    {
        let subtitle = String::from("advanced ownership, part three");
        println!("longer: {}", longer(&title, &subtitle)); // used while both live
        result = first_of(&title, &subtitle); // tied to title alone
    } // subtitle is dropped here
    println!("first_of: {result}");

    println!("{}", greeting("ana"));
    println!("{}", greeting(default_name()));
}
