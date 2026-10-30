//! Part 2: the borrow checker sees fields, but not through a method.

struct Game {
    players: Vec<String>,
    scores: Vec<u32>,
}

impl Game {
    fn players_mut(&mut self) -> &mut Vec<String> {
        &mut self.players
    }
    fn scores_mut(&mut self) -> &mut Vec<u32> {
        &mut self.scores
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

    let p = g.players_mut(); // a method borrows all of `g`
    let s = g.scores_mut();
    p.push("bo".to_string());
    s.push(0);
}
