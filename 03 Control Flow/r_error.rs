




#![allow(unused)]

#[derive(Debug)]
enum MathError {
    DivByZero,
    Other
}


fn div (x: u32, y:u32) -> Result<u32 , MathError> {
    if y == 0 {
        return Err(MathError::DivByZero);
    }
    Ok(x / y)
}



fn main() {

    let arr = [1,2,3];

    let x: Option<&i32> = arr.get(1);

    match x {
        Some(val) => println!("val = {val}"),
        None => println!("none"),
    }


    let x = 1;
    let y = 0;
    let z = div(x,y);
    match z {
        Ok(val) => println!("{val}"),
        Err(err) => println!("err = {:?}", err)
    }
}
