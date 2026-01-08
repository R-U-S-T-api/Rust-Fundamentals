

```rust



#![allow(unused)]

fn main() {
    let a: i32 = 1;
    let b: i32 = 2;


    let c: i32 = a + b;
    let d: i32 = a - b;
    let e: i32 = a * b;
    let f: i32 = a / b;
    

    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
    println!("{}", f);


    let g: i32 = a % b;

    println!("{}", g);

    let h = 1i32;
    let i = 3u64;
    let j = 1_000_000_000u32;

    let k = true && false;
    let l = true || false;
    let m = !true;

    println!("{k}");
    
    //101
    let n: u8 = 5;
    //011
    let o: u8 = 3;
    
    println!("n & b = {:03b}" , n&o);

}


```