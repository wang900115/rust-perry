#![allow(unused)]

fn borrow(s: &[i32]) {
    println!("borrow {:?}",s);
}

fn borrow_mut(s: &mut [i32]) {
    s[0] = -1;
}

fn split_at(s: &[i32], i: usize) -> (&[i32],&[i32]) {
    (&s[0..i], &s[i..])
}

fn main() {
    // borrow immutable slice
    let a: [i32; 5] = [1,2,3,4,5];
    let s: &[i32] = &a[0..2];
    borrow(s);
    println!("s = {:?}",s);


    // borrow mutable slice 
    let mut a: [i32; 5] = [1,2,3,4,5];
    let s: &mut[i32] = &mut a[0..2];
    borrow_mut(s);
    println!("s = {:?}",s);

    let a: [i32; 5] = [1,2,3,4,5];
    let (s0, s1 )= split_at(&a, 2);
} 