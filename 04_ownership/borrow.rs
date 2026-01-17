
#![allow(unused)]

fn main() {
    
    let s = String::from("rust");
    let s1 = &s;
    let s2 = &s;

    // borrow - temporarily use a value without taking ownership
    // - create reference (either mutable or immutable)
    // - doesn't move ownership

    // Immutable borrow
    let s = String::from("rust");
    let s1 = &s;
    let s2 = s1;

    println!("{s2}");
    
    // mutable reference 
    // only 1 mutable reference  
    let mut s = String::from("rust");
    let mut s1 =  &mut s;
    
    s1.push_str("");


    println!("{s1}");


    let mut s = String::from("rust");
    let s1 = &s;
    let s2 = &s;

    println!("{s1} , {s2}");


    let s = String::from("rust");
    // let s1 = &s;
    // {
    //    let s1 = s;
    // } // s1 and s is already  drop
      //
    // another command
    println!("{s}");
    std::mem::drop(s);
    println!("{s}");

}
