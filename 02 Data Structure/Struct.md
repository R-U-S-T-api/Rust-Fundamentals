```rust

#![allow(unused)]
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct Point3d(f32, f32, f32);

// empty
struct Empty;

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: u32
}

fn main() {

    let p = Point { x:1.0, y:2.0 };
    println!("{} {}", p.x , p.y);

    let p2 = Point3d(1.0, 2.0, 3.0);
    // access p2.0 p2.1 p2.2
    println!("{:?}", p2);
    println!("{} {} {}", p2.0, p2.1, p2.2);
    

    let my_empty = Empty;

    let circle = Circle {
        center: Point {x: 1.5, y: 2.7},
        radius: 2
    };
    
    println!("{:?}", circle);


    let x = 1.8;
    let y = 2.5;

    // shourtcut
    // same name x = x , y = y
    let p3 = Point {x,y};
    
    // the hussle
    let p4 = Point { x:p3.x , y: p3.y };
    println!("{:?}", p4);
    // the shorcut
    let p5 = Point { x:p3.x , ..p3};
    let p6 = Point { ..p3 };

    println!(" -------- ");
    println!("{:?}",p5);
    println!("{:?}",p6);
    println!("{}", p5.x);



    let mut p7 = Point { x , y };
    p7.x += 1.0;
    p7.y += 1.0;
    

    println!("{} {}", p7.x, p7.y);
    


}

```