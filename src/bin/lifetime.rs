#![allow(unused)]

fn longest_str<'a>(x:&'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x 
    } else {
        y 
    }
}


fn main() {}