```rust

#[allow(unused)]
fn main() {

 let msg: String = String::from("Hello rust");
 let len: usize = msg.len();
 
 println!("{msg} {len}");

let s: &str = &msg[0..5];
let s_len: usize = s.len();

println!("{s} {s_len}");


let s2: &str = r#"
	{
		"a": 1,
		"b": {"c": 2},
		"c": 3	
	}
"#;

let mut msg3: String = String::from("Dref Courption");
let s4: &str = &msg3;

msg3 += "!";

let lang = "rust";
let emoji = "emoji";

let msg6 = format!("{lang} {emoji}");


println!("{msg6}");



}
```