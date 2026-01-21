

#![allow(unused)]

fn main() {
    let v: Vec<i32> = vec![-1, 0, 1];
    let v: Vec<u32> = vec![1, 2, 3];

    let v: Vec<i32> = Vec::new();

    let v = vec![1u8,2,3];
    let v = vec![1,1,1,1,1];
    // same
    let v = vec![1u8; 5];


    println!("{:?} {}", v, v.len());

    let v = vec![1,2,3];
    
    let x = v.get(4); // 3
                    //
                    
    match x {
        Some(val) => println!("val {val}"),
        None => println!("invalid index")
    }



    let mut v = vec![1,2,3];
    v[1] = 99;

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    

    println!("{:?}", v);


    let mut v = vec![1,2,3];
    match v.pop() {
        Some(val) => println!("{:?}", val),
        None => println!("Nothing to pop"),
    }

    println!("{:?}", v);
       

    let v = vec![1,2,3,4,5];
    let s = &v[1..4];
    println!("{:?}", s);

    // push
    // update 
    // get
    // pop
    // slice
}


