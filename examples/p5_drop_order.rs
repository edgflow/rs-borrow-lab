//! Part 5: drop order: locals, fields, and the temporaries of a tail expression.

use borrow_lab::Noisy;

struct Pair {
    first: Noisy,
    second: Noisy,
}

fn tail() -> usize {
    let local = Noisy("local");
    Noisy("tail temp").0.len() + local.0.len() // Rust 2024: the temporary goes before local
}

fn main() {
    let _a = Noisy("a");
    let _b = Noisy("b");
    let pair = Pair {
        first: Noisy("pair.first"),
        second: Noisy("pair.second"),
    };
    println!("built {} and {}", pair.first.0, pair.second.0);
    println!("tail returned {}", tail());
    println!("end of main");
}
