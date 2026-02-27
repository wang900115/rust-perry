#![allow(unused)]

#[derive(Debug)]
enum MathError {
    DivByZeor,
    Other
}


fn div(x: u32, y:u32) -> Result<u32,MathError> {
    if y == 0 {
       return Err(MathError::DivByZeor);
    }
    Ok(x/y)
}


fn main() {
    // Option
    let arr = [1,2,3];
    let x: Option<&i32> = arr.get(9);
    match x {
        Some(val) => println!("val = {val}"),
        None => println!("none"),
    }

    // result
    let x = 1;
    let y=0;
    let z = div(x, y);
    match z {
        Ok(val) => println!("div = {val}"),
        Err(err) => println!("err = {:?}",err),
    }
}