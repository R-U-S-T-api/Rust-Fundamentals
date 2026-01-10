```rust



#[allow(unused)]

fn main() {
    
    // loop
    let mut i = 0;

    loop {
        println!("loop {i}");
        if i == 5 {
            break;
        }
        i += 1;
    }
    
    // while
    let mut x = 0;

    while x <= 3 {
        println!("while {x}");
        x += 1;
    }


    // for loop
    

    for z in 0..5 {
        println!("for loop {z}");
    }


    // for loop in array
    //
    //
    let arr: [u32; 3] = [1,2,3];
    for a in arr {
        print!("{a} ");
    }
    println!("");
    let n = arr.len();
    
    for i in 0..n {
        println!("{}", arr[i]);
    }

    let v = vec![1,2,3];

    for l in v.iter() {
        println!("vector {l}");
    }
    for l in v.iter() {
        println!("vector {l}");
    }


    // return loop
    let mut m = 0;

    let o = loop {
       if m == 5 {
           break 99;
       }   
       m += 1;
    };
    
    println!("{o}");



    'outer: for u in 0..5 {
        'inner: for r in 0..5 {
            println!("{u} {r}");
            if u == 1 && r == 2 {
                break 'outer;
            }
       }
    }




}





```