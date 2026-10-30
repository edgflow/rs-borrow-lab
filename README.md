# rs-borrow-lab

Advanced ownership and the borrow checker, one small example at a time: the code of the **Advanced Ownership and the
Borrow Checker** series on [@aiedgeflow](https://www.youtube.com/@aiedgeflow).

Every example is a few dozen lines you can run, change and break. Each part of the series is a range of git tags:
`p1-step-0` … for part 1, `p2-step-0` … for part 2. Check out a tag to get the code exactly as it is at that moment
of the video; some steps don't compile on purpose, to show the compiler's error.

```
cargo run --example p1_moves     # a move copies 24 bytes, never the heap buffer
cargo run --example p1_drops     # who drops what, and when
cargo run --example p1_shared    # Box and Rc
cargo run --example p1_borrows   # shared XOR mutable, and the buffer that moves
cargo run --example p1_nll       # a borrow lasts until its last use
cargo run --example p2_split     # split_at_mut, get_disjoint_mut, chunks_mut
cargo run --example p2_fields    # disjoint fields vs &mut self methods, closures
cargo run --example p2_reborrow  # implicit and explicit reborrows
cargo run --example p2_two_phase # v.push(v.len()) and where two-phase borrows stop
```

| Part | Topic | Tags |
|---|---|---|
| 1 | Ownership and the borrowing rules: moves, clones and Copy, drops, Box and Rc, shared XOR mutable, non-lexical lifetimes | `p1-step-0` … `p1-step-9` |
| 2 | Many &mut: E0499, split_at_mut, get_disjoint_mut, chunks_mut, disjoint fields, closures capturing fields, reborrows, two-phase borrows | `p2-step-0` … `p2-step-7` |

MIT licensed.
