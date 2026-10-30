//! Part 2: two mutable references into one vector.

fn main() {
    let mut v = vec![1, 2, 3, 4];
    let (left, right) = v.split_at_mut(2); // two slices that can't overlap
    let a = &mut left[0];
    let b = &mut right[1];
    *a += 10;
    *b += 10;
    println!("split_at_mut:     {v:?}");

    let [x, y] = v.get_disjoint_mut([1, 2]).unwrap(); // checked when it runs
    std::mem::swap(x, y);
    println!("get_disjoint_mut: {v:?}");

    for pair in v.chunks_mut(2) {
        pair.reverse(); // every chunk is its own &mut [i32]
    }
    println!("chunks_mut:       {v:?}");

    let same = v.get_disjoint_mut([0, 0]);
    println!("indices 0 and 0:  {:?}", same.err());
}
