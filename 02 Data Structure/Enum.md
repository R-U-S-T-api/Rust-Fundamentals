```rust



#![allow(unused)]
#[derive(Debug,PartialEq)]

enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8,u8,u8,f32),
    Hex(String),
    Hsl{h: u8, s: u8, l: u8}
}

fn main() {

    let color: Color = Color::Red;
    let color: Color = Color::Green;
    let color: Color = Color::Rgba(0,0,255,0.1);
    let color: Color = Color::Hex("#ffffff".to_string());
    let color: Color = Color::Hsl{h: 0, s: 1, l: 200};

    println!("{:?}", color);


    println!("{}", Color::Red == Color::Green);
    println!("{}", Color::Red == Color::Red);
   

    let option_x: Option<i32> = None;
    let option_y: Option<i32> = Some(-22);

    println!("{:?} {:?}", option_x, option_y);

    

    let res: Result<u32, String> = Ok(5);
    let res2: Result<u32, String> = Err("Div by 0".to_string());


    println!("{:?} {:?}", res, res2);






    
}


```