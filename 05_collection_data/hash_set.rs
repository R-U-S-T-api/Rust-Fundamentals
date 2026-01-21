
#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    
    let mut set: HashSet<u32> = HashSet::new();

    let inserted: bool = set.insert(1);
    println!("inserted: {inserted}");

    println!("{:?}", set);
    let inserted: bool = set.insert(1);
    println!("inserted: {inserted}");

    let contains: bool = set.contains(&1);
    println!("contains 1?: {contains}");


    let contains: bool = set.contains(&2);
    println!("contains 2?: {contains}");


    let contains: bool = set.contains(&3);
    println!("contains 3?: {contains}");


}
