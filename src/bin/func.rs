#![allow(unused)]

fn add(x: u32, y:u32) -> u32 {
    x+y
}

fn print() {
    println!("no output");
}

fn forever() -> ! {
    loop {}
}

fn crash() -> !{
    panic!("crash");
}

fn main() {
    let x =1;
    let y = 2;
    let z = add(x,y);
}