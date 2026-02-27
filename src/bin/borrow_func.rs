#![allow(unused)]


fn take(s: String) {
    println!("take {s}")
}

fn borrow(s: &str) {
    println!("borrow {s}")
}

fn borrow_mut(s: &mut String) {
    s.push_str("ccc");
}

fn print_len(s: String) {
    println!("len = {} ", s.len());
}


fn print_len_return_ownership(s: String) -> String {
    println!("len = {} ", s.len());
    s
}

fn print_len_borrow(s: &str) {
    println!("borrow len = {} ", s.len());
}


fn main() {
    let s = String::from("rust");
    take(s);

    // borrow immutable -> doesn't move ownership
    let s = String::from("rust");
    borrow(&s);

    // borrow mmutable
    let mut s = String::from("rust");
    borrow_mut(&mut s);

    
    // 1. Take ownership 
    let s = String::from("rust");
    print_len(s);

    // 2. Return ownership
    let s = String::from("rust");
    let s = print_len_return_ownership(s);

    // 3. Borrows 
    let s = String::from("rust");
    print_len_borrow(&s);

}