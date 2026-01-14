

#[allow(unused)]


fn main() {

    let x: Option<i32> = Some(2); 

    // let v = x.unwrap();
    let v = x.expect("x is none");
    println!("v = {v}");


    // results
    let x = 1;
    let y = 1;
    let z:Result<u32, String> = Ok (x/y);
    let z:Result<u32, String> = Err("div by 0".to_string());
    
    let v = z.unwrap(); 

    println!("v = {v}")




}
