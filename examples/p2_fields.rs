//! Part 2: the borrow checker sees fields, but not through a method.

struct Game {
    players: Vec<String>,
    scores: Vec<u32>,
}

impl Game {
    /// One method, one borrow of `self`, split into its two fields inside.
    fn parts_mut(&mut self) -> (&mut Vec<String>, &mut Vec<u32>) {
        (&mut self.players, &mut self.scores)
    }
}

fn main() {
    let mut g = Game {
        players: vec![],
        scores: vec![],
    };

    let p = &mut g.players; // two fields: two disjoint borrows
    let s = &mut g.scores;
    p.push("ana".to_string());
    s.push(0);

    let (p, s) = g.parts_mut(); // the method hands out both fields at once
    p.push("bo".to_string());
    s.push(0);

    let mut bump = || g.scores[0] += 1; // the closure captures only g.scores
    g.players.push("cy".to_string()); // so g.players is still free
    bump();
    println!("players {:?}, scores {:?}", g.players, g.scores);
}
