//! Part 3: a struct that borrows: whose data does a token point into?

struct Lexer<'src> {
    src: &'src str,
    pos: usize,
}

impl<'src> Lexer<'src> {
    fn new(src: &'src str) -> Self {
        Lexer { src, pos: 0 }
    }

    /// The next word of the source, or None at the end.
    fn next_token(&mut self) -> Option<&'src str> {
        let rest = &self.src[self.pos..];
        let start = rest.find(|c: char| !c.is_whitespace())?;
        let rest = &rest[start..];
        let len = rest.find(char::is_whitespace).unwrap_or(rest.len());
        self.pos += start + len;
        Some(&rest[..len])
    }
}

fn main() {
    let source = String::from("let total = price * 2");
    let mut lexer = Lexer::new(&source);
    let first = lexer.next_token();
    let second = lexer.next_token();
    println!("{first:?} {second:?}");

    let mut tokens = vec![];
    while let Some(token) = lexer.next_token() {
        tokens.push(token); // each token borrows `source`, not `lexer`
    }
    println!("{tokens:?}");
}
