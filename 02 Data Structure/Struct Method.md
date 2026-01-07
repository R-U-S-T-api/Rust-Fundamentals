```rust


#![allow(unused)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

impl Point {
    fn zero () -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    fn move_to(&mut self, x: f32, y: f32){
        self.x = x;
        self.y = y;
    }

    fn dist(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let mut p = Point::zero();
    println!("{:?}", p);

    p.move_to(10.5, 20.6);
    println!("{:?}", p);

    let d = p.dist();
    println!("{:?}",d);

}

```