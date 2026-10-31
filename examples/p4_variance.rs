//! Part 4: subtyping and variance: when a longer lifetime can stand in for a shorter one.

use std::cell::Cell;

/// Returns the first word; it lives as long as the words in the list.
fn first<'a>(words: &[&'a str]) -> &'a str {
    words[0]
}

/// Writes `src` into the variable that `dst` points to.
fn overwrite<'a>(dst: &mut &'a str, src: &'a str) {
    *dst = src;
}

/// Stores `s` in the cell, through a shared reference.
fn set<'a>(cell: &Cell<&'a str>, s: &'a str) {
    cell.set(s);
}

fn main() {
    let local = String::from("local");
    let fixed: Vec<&'static str> = vec!["static", "words"];
    println!("{}", first(&fixed)); // here 'a is 'static

    let mut mixed: Vec<&str> = fixed; // a Vec<&'static str> used as a Vec of shorter &str
    mixed.push(&local); // so it can hold a shorter-lived word too
    println!("{mixed:?} {}", first(&mixed));

    let mut current: &str = "hello"; // no 'static: the compiler picks the lifetime
    let shared: Cell<&str> = Cell::new("static");
    {
        let short = String::from("short-lived");
        overwrite(&mut current, &short);
        set(&shared, &short);
        println!("{current} {}", shared.get()); // used while short is alive
    }
}
