#![allow(unused)]

use std::collections::HashSet;

fn main() {
    // HashSet 
    let mut set: HashSet<u32> = HashSet::new();
    let inserted = set.insert(1);
    
    let contains = set.contains(&1);
}