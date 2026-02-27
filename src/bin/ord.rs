#![allow(unused)]

use std::ops::Add;

struct Point<T> {
    x: T,
    y: T, 
}

impl<T>  Add for Point<T> 
where T: Add<Output=T>,
{
    type Output = Point<T>;
    fn add(self, rhs: Point<T>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {}