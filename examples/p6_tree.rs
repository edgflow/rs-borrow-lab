//! Part 6: a tree of MyRc nodes: children owned by their parent, and a MyWeak back up.

use std::cell::RefCell;

use borrow_lab::Noisy;
use borrow_lab::my_rc::{MyRc, MyWeak};

struct Node {
    name: Noisy,
    parent: RefCell<Option<MyWeak<Node>>>,
    children: RefCell<Vec<MyRc<Node>>>,
}

fn node(name: &'static str) -> MyRc<Node> {
    MyRc::new(Node {
        name: Noisy(name),
        parent: RefCell::new(None),
        children: RefCell::new(Vec::new()),
    })
}

fn main() {
    let root = node("root");
    let leaf = node("leaf");
    root.children.borrow_mut().push(leaf.clone()); // root owns leaf
    *leaf.parent.borrow_mut() = Some(MyRc::downgrade(&root)); // leaf only points back
    let upgraded = leaf.parent.borrow().as_ref().and_then(|w| w.upgrade());
    let parent = upgraded.map(|p| p.name.0);
    println!("leaf's parent: {}", parent.unwrap_or("none"));
    let (r, l) = (MyRc::strong_count(&root), MyRc::strong_count(&leaf));
    println!("root count: {r}, leaf count: {l}");
    println!("end of main");
}
