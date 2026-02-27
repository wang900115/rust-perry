#![allow(unused)]

// iter - borrows and returns a iterator that return &T 
// into_iter - takes ownership and returns a iterator that may return T, &T, &mut T
// iter_mut - return &mut T
fn main() {
    let mut vals = vec![1,2,3];

    for mut v in vals.iter_mut() {
        *v += 1;
        println!("{}",*v);
    }
}