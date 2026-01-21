#![allow(unused)]



use std::collections::HashMap;

fn main() {


    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert("red".to_string(), 100);
    scores.insert("green".to_string(), 100);
    
    let val: Option<&u32> = scores.get("red");
    println!("{:?}", val);
    let val2: Option<&u32> = scores.get("pink");
    println!("{:?}", val2);


    // to update just overwrite

    scores.insert("green".to_string(), 200);
    let val3: Option<&u32> = scores.get("green");

    println!("{:?}",val3);


    let v5: &mut u32 = scores.entry("blue".to_string()).or_insert(0);
    *v5 += 250;
    
    let val: Option<&u32> = scores.get("blue");
    println!("blue val: {:?}", val);


}
