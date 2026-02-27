#![allow(unused)]


fn f<T: Sized>(x: T) {}
fn g<T: ?Sized>(x: &T) {}


fn main() {}