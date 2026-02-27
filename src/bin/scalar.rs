#![allow(unused)]

fn main() {
    // sign integers
    let i0: i8 = 0;
    let i1: i16 = 1;
    let i2: i32 = 1; 
    let i3: i64 = 1; 
    let i4: i128 = 1;
    let i5: isize = 1;

    // unsign integers
    let u0: u8 = 1; 
    let u1: u16 = 1;
    let u2: u32 = 1;
    let u3: u64 =1 ;
    let u4: u128 = 1;
    let u5: usize = 1;

    // floats 
    let f0: f32 = 0.01; 
    let f1: f64 = 0.01;

    // boolean
    let b: bool = true;

    // characters
    let c: char = 'c';

    // type conversion 
    let i: i32 = 1;
    let u: u32 = i as u32;
    let x: u32 = u + (i as u32);

    // min and max
    let min_i: i32 = i32::MIN;
    let max_i: i32 = i32::MAX;

}