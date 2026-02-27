#![allow(unused)]

fn main() {
    // Vec<T> 
    let v: Vec<i32> = vec![-1,0,1];
    let v: Vec<u32> = Vec::new();
    let v = vec![1u8, 2,3];
    let v = vec![1u8;5];
    // get 
    let v = vec![1,2,3];
    let x = v[0];
    let x = v.get(0);
    // update
    let mut v = vec![1,2,3];
    v[1] = 99;

    // push 
    let mut v: Vec<u32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // pop 
    let mut v = vec![1,2,3];
    match v.pop() {
        Some(val) => println!(),
        None => println!(),
    }

    // slice 
    let v = vec![1,2,3,4,5];
    let s = &v[1..4];


} 