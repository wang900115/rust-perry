#![allow(unused)]

enum Animal {
    Cat,
    Dog,
    Mouse,
    Duck,
}

fn main() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }

    match x {
        1 | 2 | 3 => println!("1 or 2 or 3"),
        _ => println!("other"),
    }

    match x {
        1..10 => println!("1 ~ 10"),
        _ => println!("other"),
    }

    match x {
        i @1..=10 => println!("{i}"),
        _ => println!("other"),
    }

    let animal = Animal::Cat;

    let animal_sound = match animal {
        Animal::Cat => "meow",
        Animal::Dog => "woof",
        Animal::Duck => "quack",
        _ => "",
    };
}