#![allow(unused)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    // associated func 
    fn zero() -> Self {
        Self {
            x: 0.0, 
            y: 0.0,
        }
    }
    // methods func
    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn dist(&self) -> f32 {
        (self.x * self.x + self.y *self.y).sqrt()
    }
}

fn main(){
    let mut p = Point::zero();
    p.move_to(2.0,1.0);
    let d = p.dist();
}