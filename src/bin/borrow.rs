#![allow(unused)]

fn main() {

    // immutable borrow 

    let s = String::from("rust");
    let s1 = &s;

    // mutable borrow (only one mutable reference at a time)
    let mut s = String::from("rust");
    let s1 = &mut s;

    // cannot borrow immutable and mutable simultaneously
    let mut s = String::from("rust");
    let s1 = &s;
    // let s2 = &mut s;


    // reference must not outlive the value
    let  s = String::from("rust");
    let s1 = &s;
    {
        let s1 =s;
    } // s1 is dropped
    // std::mem::drop(s);
}

// fn f(s: String) -> &String {
//     &s
// }