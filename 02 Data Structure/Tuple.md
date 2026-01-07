```rust


#![allow(unused)]
fn main() {
	// basic tuple
	let t: (bool, u32, char) = (true, 1, 'c');
	
	// a will be true
	// b will be 1
	// c will be 'c'
	let (a,b,c,) = t;
	
	// skipin
	// b will be 1
	let (_,b,_) = t;
	
	// empty tuple	
	let e = ();
	
	//nested
	let nested = ( (1.23, 'a') , (true, 1u32, 'b') , () );
	
	// destructure  tuple
	println!("t = {}, {}, {}", t.0, t.1, t.2);
	
	// destructure nested tuple
	println!("nested tuple: {} , {}", nested.0.1, nested.1.1);
	
	
	
	
}

```