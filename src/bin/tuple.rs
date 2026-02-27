#![allow(unused)]

fn main() {
    let t : (bool, u32, char) = (true, 1, 'c');
    // t.0
    // t.1
    // t.2


    // destructe
    let (a,b,c) = t;
    // ignore 
    let (_,b,_) = t;
    // empty 
    let t = ();

    // nested 
    let nest = ((1.23,'a'),(1u32, 'b'));
}