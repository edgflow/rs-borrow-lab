//! Part 3: lifetime bounds: 'a: 'b, and T: 'static.

use std::fmt::Display;
use std::thread;

/// Returns `x`, but promises a reference that lives for 'b.
fn pick<'a: 'b, 'b>(x: &'a str, _y: &'b str) -> &'b str {
    x
}

/// A thread may outlive the caller, so `value` must not borrow anything short-lived.
fn spawn_print<T: Display + Send + 'static>(value: T) -> thread::JoinHandle<()> {
    thread::spawn(move || println!("from a thread: {value}"))
}

fn main() {
    let long = String::from("lives long");
    {
        let short = String::from("short");
        println!("{}", pick(&long, &short));
    }

    let local = String::from("a local string");
    spawn_print(local).join().unwrap(); // moved in: an owned String is 'static
    spawn_print("a string literal").join().unwrap();
    spawn_print(42).join().unwrap();
}
