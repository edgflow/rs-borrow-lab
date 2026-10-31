//! Part 5: a temporary that lives for a whole match: a MutexGuard.

use std::sync::Mutex;

fn main() {
    let scores = Mutex::new(vec![3, 9, 4]);

    match scores.lock().unwrap().first() {
        // the guard is a temporary of the scrutinee: it lives until the match ends
        Some(first) => {
            let locked = scores.try_lock().is_err(); // lock again, without waiting
            println!("first = {first}, still locked inside the match: {locked}");
        }
        None => println!("empty"),
    }

    let first = scores.lock().unwrap().first().copied(); // the guard dies at this ;
    if let Some(first) = first {
        let locked = scores.try_lock().is_err();
        println!("first = {first}, locked after a let: {locked}");
    }
}
