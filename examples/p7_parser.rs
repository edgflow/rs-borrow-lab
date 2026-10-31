//! Part 7: an interner that hands out names from an arena.

use std::collections::HashSet;

use borrow_lab::arena::Arena;

/// One &'arena str per distinct name: equal names, the same pointer.
struct Interner<'arena> {
    arena: &'arena Arena,
    names: HashSet<&'arena str>,
}

impl<'arena> Interner<'arena> {
    fn intern(&mut self, name: &str) -> &'arena str {
        if let Some(&known) = self.names.get(name) {
            return known; // a copy of the &'arena str: no borrow of self.names
        }
        let name = self.arena.alloc(name);
        self.names.insert(name);
        name
    }
}

fn main() {
    let arena = Arena::default();
    let mut names = Interner {
        arena: &arena,
        names: HashSet::new(),
    };
    let a = names.intern("lang");
    let b = names.intern("year");
    let c = names.intern("lang"); // while a and b are still in use
    println!("{a} {b} {c}, same pointer: {}", std::ptr::eq(a, c));
}
