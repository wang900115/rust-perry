#![allow(unused)]

fn swap<A,B>(t: (A, B)) -> (B, A) {
    (t.1, t.0)
}

fn main() {
    let t = (1,2);
    let s = swap(t);

}