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
cargo run --example p5_temps     # temporaries, E0716, let x = &temporary
cargo run --example p5_drop_order # locals, fields, tail expressions (Rust 2024)
cargo run --example p5_match_lock # a MutexGuard that lives for a whole match
cargo run --example p5_partial   # partial moves, E0509, mem::take
cargo run --example p5_init      # E0381, MaybeUninit field by field
cargo run --example p6_box       # MyBox<T>: alloc, Deref, Drop, DerefMut, deref coercion
cargo run --example p6_dropck    # drop check: Box vs MyBox (E0597)
cargo run --example p6_rc        # MyRc<T>: a count next to the value
cargo run --example p6_tree      # a cycle that leaks, and MyWeak
cargo +nightly miri test --lib   # the tests, under Miri (rustup +nightly component add miri)
cargo run --example p7_self_ref  # a struct that borrows itself (E0505, E0515), and indices
cargo run --example p7_pin       # a pointer to its own field, a move, and Pin + PhantomPinned (E0277)
cargo run --example p7_nll3      # NLL problem case #3 (E0502), Polonius, the entry API
cargo run --example p7_parser    # an arena, an interner and a parser with two lifetimes (E0597)
```

| Part | Topic | Tags |
|---|---|---|
| 1 | Ownership and the borrowing rules: moves, clones and Copy, drops, Box and Rc, shared XOR mutable, non-lexical lifetimes | `p1-step-0` … `p1-step-9` |
| 2 | Many &mut: E0499, split_at_mut, get_disjoint_mut, chunks_mut, disjoint fields, closures capturing fields, reborrows, two-phase borrows | `p2-step-0` … `p2-step-7` |
| 3 | Lifetimes in signatures: elision, E0106, one lifetime for two inputs (E0597), two lifetimes, E0515 and owned results, a struct that borrows (Lexer<'src>, E0499, E0505), 'a: 'b, T: 'static | `p3-step-0` … `p3-step-10` |
| 4 | Subtyping, variance and higher-ranked lifetimes: &'static T as &'a T, covariance of Vec, invariance of &mut T and Cell<T> (E0597), for<'a> (E0597, not general enough), closures returning references, Box<dyn Fn(&str) -> &str> | `p4-step-0` … `p4-step-8` |
| 5 | Temporaries and drop order: E0716, lifetime extension, drop order of locals, fields and tail temporaries, a lock held by a match, partial moves (E0382), E0509 and mem::take, partial initialization (E0381) and MaybeUninit | `p5-step-0` … `p5-step-9` |
| 6 | Build your own smart pointer: MyBox<T> with alloc, Deref, Drop and DerefMut (E0609), deref coercion, drop check (E0597), MyRc<T> and MyWeak<T>, a cycle that leaks, tests under Miri (memory leaked, an allocation of size 0) | `p6-step-0` … `p6-step-10` |
| 7 | Self-referential structs and the borrow checker's limits: a struct that borrows itself (E0505, E0515) and indices, a pointer to its own field, Pin and PhantomPinned (E0277), NLL problem case #3 (E0502) and Polonius, an arena, an interner and a parser with two lifetimes (E0597) | `p7-step-0` … `p7-step-10` |

MIT licensed.
