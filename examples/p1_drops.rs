//! Part 1: every value is dropped exactly once, by its last owner.

use borrow_lab::Noisy;

fn inspect(n: &Noisy) {
    println!("  inspect {}: borrowed, nothing to drop", n.0);
}

fn consume(n: Noisy) {
    println!("  consume {}: owned, dropped when this function ends", n.0);
}

fn main() {
    let config = Noisy("config");
    let buffer = Noisy("buffer");
    let log = Noisy("log");
    inspect(&buffer);
    consume(buffer);
    println!("back in main, still owning {} and {}", config.0, log.0);
    {
        let _temp = Noisy("temp");
        println!("  the inner scope ends");
    }
    println!("main ends");
}
