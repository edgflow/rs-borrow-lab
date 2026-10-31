//! Part 7: a parser that borrows its source and interns names in an arena.

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

/// Reads `name=value` pairs: values are slices of the source, names come from the interner.
struct Parser<'src, 'arena> {
    rest: &'src str,
    names: Interner<'arena>,
}

impl<'src, 'arena> Parser<'src, 'arena> {
    fn next_pair(&mut self) -> Option<(&'arena str, &'src str)> {
        let (pair, rest) = self.rest.split_once(' ').unwrap_or((self.rest, ""));
        self.rest = rest;
        let (name, value) = pair.split_once('=')?;
        Some((self.names.intern(name), value))
    }
}

fn main() {
    let arena = Arena::default();
    let mut names = Vec::new();
    {
        let source = String::from("lang=rust year=2015 lang=c");
        let mut parser = Parser {
            rest: &source,
            names: Interner {
                arena: &arena,
                names: HashSet::new(),
            },
        };
        while let Some((name, value)) = parser.next_pair() {
            println!("{name} = {value}");
            names.push(name); // keep the names
        }
    } // the source is gone here
    println!("names: {names:?}");
    println!("same pointer: {}", std::ptr::eq(names[0], names[2]));
}
