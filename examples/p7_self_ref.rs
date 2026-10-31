//! Part 7: a struct that borrows from itself, and why Rust says no.

struct Document<'a> {
    text: String,
    words: Vec<&'a str>, // slices of text: a field of the same struct
}

impl<'a> Document<'a> {
    fn new(text: String) -> Document<'a> {
        let mut doc = Document {
            text,
            words: Vec::new(),
        };
        doc.words = doc.text.split(' ').collect();
        doc
    }
}

fn main() {
    let doc = Document::new(String::from("the quick brown fox"));
    println!("{} words", doc.words.len());
}
