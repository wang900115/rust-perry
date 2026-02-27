#![allow(unused)]

struct Point {
    x: f32,
    y: f32,
}

struct Point3d(f32, f32, f32);

struct Empty;

struct Circle {
    center: Point,
    radius: u32, 
}

fn main() {
    // create 
    let p = Point{x: 1.0, y:2.0};
    // p.x
    // p.y
    let p = Point3d(1.0, 2.0, 3.0);
    // p.0 
    // p.1
    // p.2
    let empty = Empty;

    let circle = Circle{
        center: Point { x: 1.0, y: 2.0 },
        radius:1,
    };

    let circle1 =  Circle{
        center: Point { x: 1.0, y: circle.center.y },
        ..circle
    };

    let mut p = Point {x: 0.0, y:0.0};
    p.x += 1.0;
    p.y += 1.0;
}