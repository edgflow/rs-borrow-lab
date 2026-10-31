//! Part 7: NLL problem case #3: a borrow returned on one path only.

use std::collections::HashMap;

fn get_or_insert(map: &mut HashMap<u32, String>, key: u32) -> &String {
    if let Some(value) = map.get(&key) {
        return value; // this path returns the borrow
    }
    map.insert(key, format!("value {key}")); // map still counts as borrowed here
    map.get(&key).unwrap()
}

fn main() {
    let mut map = HashMap::new();
    println!("{}", get_or_insert(&mut map, 7));
    println!("{}", get_or_insert(&mut map, 7));
}
