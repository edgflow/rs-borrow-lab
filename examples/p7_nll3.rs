//! Part 7: NLL problem case #3: a borrow returned on one path only; the entry API instead.

use std::collections::HashMap;

fn get_or_insert(map: &mut HashMap<u32, String>, key: u32) -> &String {
    map.entry(key).or_insert_with(|| format!("value {key}")) // one lookup, one borrow
}

fn main() {
    let mut map = HashMap::new();
    println!("{}", get_or_insert(&mut map, 7));
    println!("{}", get_or_insert(&mut map, 7));
}
