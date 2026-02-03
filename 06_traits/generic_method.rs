

#![allow(unused)]



#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Point2<T> {
    x: T,
    y: T,
}

impl<T> Point2 <T> {
    fn new (x: T, y: T) -> Self {
        Self { x,y }
    }

    fn move_to(&mut self, x:T, y: T) {
        self.x = x;
        self.y = y;
    }
}


impl Point {
    fn new(x: u32, y: u32) -> Self {
        Self {x,y}
    }

    fn move_to(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}


fn main() {
    let mut p: Point = Point::new(1,2);
    p.move_to(2,3);
    println!("{:?}", p);


    let mut p2: Point2<i32> = Point2::new(3,4);
    p2.move_to(5, 2);
    println!("{:?}", p2);
}




