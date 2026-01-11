```rust



#![allow(unused)]
fn main() {

    let x: Option<u32> = Some(123);
    // let x: Option<u32> = None;

    match x {
        Some(v) => println!("Some {v}"),
        _ => {}
    }

    if let Some(v) = x {
        println!("if let {v}");
    }

    let Some(v) = x else {
        panic!("x is none");
    };
    

    println!(" v = {v}");
}


```