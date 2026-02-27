#![allow(unused)]


fn main() {
    // String 
    let msg: String = String::from("Hello Rust");
    let len: usize = msg.len();

    // str 
    let s: &str = &msg[0..5];
    let len: usize = s.len();
}