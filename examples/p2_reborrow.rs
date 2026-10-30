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
    ignore(r); // generic: T = &mut i32, so `r` itself is moved in
    bump(r);
    println!("n = {n}");
}
