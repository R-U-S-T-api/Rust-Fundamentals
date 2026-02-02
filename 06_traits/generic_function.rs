


#![allow(unused)]

use std::cmp::PartialOrd;

fn swap<A,B>(t: (A, B)) -> (B,A) {
    (t.1 , t.0)
}

fn max<T: PartialOrd>(s: &[T]) -> Option<&T> {

    if s.len() == 0 {
        return None;
    }

    let mut largest = &s[0];
    for item in s {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)

}


fn main() {

    let t: (u32, u32) = (1,2);
    let s = swap(t);
    println!("{:?}",s);


    let gene: (i32, u32) = (-1,2);
    let s2 = swap(gene);
    println!("{:?}", s2);

    let nums = vec![33, 1, 22, 35, 8, 77, 51, 58];
    let largest = max(&nums);
    println!("largest number {:?}", largest.unwrap());

    let lttr = vec!['a','b','c','d','e','f',];
    let letter_larg = max(&lttr);
    println!("largest letter {:?}", letter_larg.unwrap());


}
