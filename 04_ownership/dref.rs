
#![allow(unused)]

fn modify(s: &mut String) {
    *s += "?";
}

fn main() {

    let mut s = String::from("rust");
    let s1 = &mut s;

    *s1 += "?";
    println!("{s1}");


    let mut s = String::from("rust");
    modify(&mut s);

    println!("if this run its not take ownership");



    let x = 1;
    let y = &x;
    let z = &z;

    let w = y + z;
    println!("w = {w}");
        

}
