#![allow(unused)]

fn main() {
    // Option
    let x: Option<i32> = Some(3);
    // match x {
    //     Some(val) => println!("val = {val}"),
    //     None => println!("none"),
    // }
    let v = x.expect("x is none");
    println!("val = {v}");

    // result
    let x = 1;
    let y=0;
    let z: Result<u32, String>  = Ok(x/y);
    match z {
        Ok(val) => println!("div = {val}"),
        Err(err) => println!("err = {:?}",err),
    }
}