#![allow(unused)]

use std::{thread, thread::JoinHandle, time::Duration};
/*
    - good for CPU bound computation
    - limited by memory and os thread limits
*/


fn main() {
    let h1: JoinHandle<_> = thread::spawn(|| {
        for i in 0..5 {
            println!("{i}");
            thread::sleep(Duration::from_millis(10));
        }
    });
    let h2: JoinHandle<_> = thread::spawn(|| {
        for i in 0..5 {
            println!("{i}");
            thread::sleep(Duration::from_millis(10));
        }
    });
    h1.join().unwrap();
    h2.join().unwrap();


    let v = vec![1u32, 2u32, 3u32];
    let h = thread::spawn(move || {
        println!("v: {:?}",v)
    });
    h.join().unwrap();


    let h = thread::spawn(|| {
        return 1u32;
    });

    match h.join() {
        Ok(val) => println!("v:{val}"),
        Err(err) => {},
    };
}