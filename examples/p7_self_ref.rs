//! Part 7: a struct that borrows from itself, and why Rust says no; indices instead.

use std::ops::Range;

struct Document {
    text: String,
    words: Vec<Range<usize>>, // where each word is: byte ranges, not references
}

impl Document {
    fn new(text: String) -> Document {
        let mut words = Vec::new();
        let mut start = 0;
        for word in text.split(' ') {
            words.push(start..start + word.len());
            start += word.len() + 1; // the word, and the space after it
        }
        Document { text, words }
    }

    fn word(&self, i: usize) -> &str {
        &self.text[self.words[i].clone()] // a slice, made when you ask for it
    }
}

fn main() {
    let doc = Document::new(String::from("the quick brown fox"));
    let moved = doc; // a move is fine now: a range doesn't point anywhere
    println!(
        "{} words, the third is {:?}",
        moved.words.len(),
        moved.word(2)
    );
}
