#![allow(unused)]

// static:
// - function to call is know at compile time 
// - monomorphization 

// dynamic:
// - function to call is know at run time
// - vtable lookup



#[derive(Debug)]
struct A; 

#[derive(Debug)]
struct B;

trait F {
    fn f(&self);
}

impl F for  A {
    fn f(&self) {
        println!("{:?}",self);
    }
}

impl F for  B {
    fn f(&self) {
        println!("{:?}", self);
    }
}

fn static_dispatch<T: F>(t: &T) {
    t.f();
}

fn dynamic_dispatch(t: &dyn F) {
    t.f();
}

fn main() {}