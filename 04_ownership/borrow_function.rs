
#![allow(unused)]

fn take(s: String) {
    println!("take {s}");
}


// this work too rust is smart enough 
// fn borrow(s: &str) {e
fn borrow(s: &String) {
    println!("borrow {s}");
}


fn borrow_mut(s: &mut String) {
    s.push_str("…¬µµ˜∆¬˚∆˙¬˚∆¬˚∆ˆ¨ˆø¨¨¥†¥˙∆©∆√∫∫√∫˜ç≈≈∂ƒß†∂¥œ∑´¨¥†≈©Ωç√µΩb");
}

fn print_len(s: String) {
    println!("length = {}", s.len());
}

fn print_len_return_own(s: String) -> String {
    println!("length = {}", s.len());
    s
}

fn print_len_borrow(s : &String) {
    println!("borrow length = {}", s.len());
}

fn main() {
    let s = String::from("rust");
    take(s);
    println!("--------");

    // immutable -> dosnt move borrow ownership
    let s = String::from("rust");
    borrow(&s);
    println!("{s}");

    println!("--------");

    let mut s = String::from("rust");
    borrow_mut(&mut s);
    println!("{s}");

    println!("--------");
    let s = String::from("rust");
    print_len(s);
    // 
    // if u println("{s}")
    // error s already taken
    //

    println!("--------");
    let s = String::from("rust");
    let s = print_len_return_own(s);
    println!("{s}");

    println!("--------");
    let s = String::from("rust");
    print_len_borrow(&s);
    // take note no return on print_len_borrow
    // because alreadyy refence only
    // dint take ownership
    println!("{s}");

}

