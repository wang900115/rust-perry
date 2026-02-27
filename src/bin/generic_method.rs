#![allow(unused)]

#[derive(Debug)] 
struct Point<T>{
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y:T) -> Self {
        Self {x,y}
    }
    
    fn move_to(&mut self, x: T, y:T) {
        self.y = y;
        self.x = x ;
    }
}

fn main()  {}