#![allow(unused)]

fn main() {
    let x: Option<u32> = Some(123);

    // if let 
    if let Some(v) = x {
        println!("if let {v}");
    }

    // let else 

    let Some(v) = x else {
        // diverge 
        panic!("x is none");
    };

}