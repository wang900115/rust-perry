#![allow(unused)]


trait A {}
trait B {}
trait C {}

impl A for u32 {}
impl B for u32 {}
impl C for u32 {}
impl A for i32 {}

fn c<T: A>(x: T) {}
fn m<T: A + B>(x: T) {}


fn main() {


}