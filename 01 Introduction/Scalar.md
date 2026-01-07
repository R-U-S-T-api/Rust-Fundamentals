

```rust


#![allow(unused)]

fn main() {


    let i0: i8 = 1; // -128 to 127
    let i1: i16 = 1; // -32768 to 32767
    let i2: i32 = 1; // -2147483648 to 2147483647
    let i3: i64 = 1; // -923372036854775808 to 9223372036854775807
    let i4: isize = 1; // depending on computer architecture



    let u0: u8 = 1; // 0 to 255
    let u1: u16 = 1; // 0 to 65535
    let u2: u32 = 1; // 0 4294967295
    let u3: u64 = 1; // 0 18446744073709551615
    let u4: usize = 1; // depending on computer architecture

    let f0: f32 = 0.01;
    let f1: f64 = 0.01;


    let b: bool = true;

    let c: char = 'c';
    let e: char = '†';


    // type conversion

    let i: i32 = 1;
    let u: u32 = i as u32;
    let x: u32 = u + (i as u32);


    // min max
    let min_i: i32 = i32::MIN;
    let max_i: i32 = i32::MAX;

    println!("i32 min: {min_i}");
    println!("i32 min: {max_i}");


    let min_char: char = char::MIN;
    let max_char: char = char::MAX;

    println!("char min: {min_char}");
    println!("char max: {max_char}");


    // overflow
    let mut overflow: u32 = u32::MAX;
    overflow += 1;
    println!("{overflow}");
    // additional 
    // can run overflow variable using
    // --release
    // example
    // cargo run --bin scalar --release

    // checking overflow
    // Some(X) if ok no overflow
    // NONE if error 
    let c_overflow = u32::checked_add(u32::MAX, 1);
    println!("checked_add: {:?}", c_overflow);
    

    let w_overflow = u32::wrapping_add(u32::MAX, 1);
    println!("wrapping_add: {:?}", w_overflow);



}




```