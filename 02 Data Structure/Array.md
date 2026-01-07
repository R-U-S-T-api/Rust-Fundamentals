```rust

#![allow(unused)]

fn main() {
	let ar: [u32; 3] = [1,2,3];
	println!("{:?}", ar);
	
	let ar2: [u32; 10] = [0; 10];
	println!("{:?}", ar2);
	
	
	let ar3: [i32; 10] = [-1, 1, -2, 2, -3, 3, -4, 4, -5, 5];
	
	// 0 until 2 element 	
	// -1 1
	let ar4 = &ar3[0..3];
	// -3 3 -4 4 -5
	let ar5 = &ar3[4..9]
	
	let ar6 = &ar3[..]
	
	
}



```