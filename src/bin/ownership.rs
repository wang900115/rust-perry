#![allow(unused)]

fn f(s: String) {}

fn main() {
    
    // 1. each value has an owner 
    // Owner of "rust" is s 
    let s = String::from("rust");
    // Owner of -1 is i 
    let i = -1;
    // 2. there can only be one owner at time 
    let s = String::from("rust");
    let s1 = s; // Owner of "rust" is s1
    let s2 = s1;// Owner of "rust" is s2

    let i = -1; // Owner of -1 is i
    let i1 = i; // Owner of -1 is i1 copyover

    // 3. when the owner goes out of scape, the value will be dropped
    let s = String::from("rust");
    if (true) {
        s ;
    } // s is dropped

    let s = String::from("rust");
    {
        let s1 = s; 
    } // s1 is dropped

}