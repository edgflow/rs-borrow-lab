//! Part 2: `&mut T` is not Copy, yet you can pass it twice: reborrows.

fn bump(x: &mut i32) {
    *x += 1;
}

fn ignore<T>(_value: T) {}

fn main() {
    let mut n = 0;
    let r = &mut n;
    bump(r); // the compiler passes `&mut *r`: a fresh, shorter borrow
    bump(r); // so `r` is still usable
    ignore(&mut *r); // an explicit reborrow: `r` stays ours
    bump(r);

    let rr = &mut *r; // a reborrow in a variable
    *rr += 10; // while `rr` is in use, `r` is frozen
    *r += 100; // fine: `rr` is never used again
    println!("n = {n}");
}
