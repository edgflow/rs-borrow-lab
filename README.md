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
cargo run --example p3_return    # elision, E0106, 'a, E0597, E0515, &'static str
cargo run --example p3_lexer     # a Lexer<'src> whose tokens borrow the source
cargo run --example p3_bounds    # 'a: 'b and T: 'static
cargo run --example p4_variance  # covariance, and why &mut T and Cell<T> are invariant
cargo run --example p4_hrtb      # for<'a>: closures that work for every lifetime
cargo run --example p4_closures  # closures that return references, Box<dyn Fn(&str) -> &str>
```

| Part | Topic | Tags |
|---|---|---|
| 1 | Ownership and the borrowing rules: moves, clones and Copy, drops, Box and Rc, shared XOR mutable, non-lexical lifetimes | `p1-step-0` … `p1-step-9` |
| 2 | Many &mut: E0499, split_at_mut, get_disjoint_mut, chunks_mut, disjoint fields, closures capturing fields, reborrows, two-phase borrows | `p2-step-0` … `p2-step-7` |
| 3 | Lifetimes in signatures: elision, E0106, one lifetime for two inputs (E0597), two lifetimes, E0515 and owned results, a struct that borrows (Lexer<'src>, E0499, E0505), 'a: 'b, T: 'static | `p3-step-0` … `p3-step-10` |
| 4 | Subtyping, variance and higher-ranked lifetimes: &'static T as &'a T, covariance of Vec, invariance of &mut T and Cell<T> (E0597), for<'a> (E0597, not general enough), closures returning references, Box<dyn Fn(&str) -> &str> | `p4-step-0` … `p4-step-8` |

MIT licensed.
