#![allow(unused)]

use std::collections::HashMap;

fn main() {
    // initialize
    let mut scores: HashMap<String, u32> = HashMap::new();
    // Insert
    scores.insert("red".to_string(), 100);
    scores.insert("green".to_string(), 100);
    // Get 
    let val = scores.get("red");


    // Update 
    scores.insert("green".to_string(), 200);


    // Upsert 
    let v : &mut u32 = scores.entry("blue".to_string()).or_insert(0);
    *v += 200;

}