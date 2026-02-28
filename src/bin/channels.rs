#![allow(unused)]

use std::sync::mpsc::{self, TryRecvError};
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::Duration;


fn main() {
    let (tx, rx): (Sender<String>,Receiver<String>) = mpsc::channel();
    // std::mem::drop(rx); 
    thread::spawn( move || {
        thread::sleep(Duration::from_millis(100));
        tx.send("hello 1".to_string()).unwrap();
        tx.send("hello 2".to_string()).unwrap();
        tx.send("hello 3".to_string()).unwrap();
        tx.send("hello 4".to_string()).unwrap();

    });

    loop {
        match rx.try_recv() {
            Ok(msg) => println!("res: {:#?}",msg),
            Err(TryRecvError::Empty) => {
                println!("no message");
            } 
            Err(TryRecvError::Disconnected) => {
                println!("disconnected");
                break;
            }
        }
        thread::sleep(Duration::from_millis(10));
    };

}