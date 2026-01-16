

#![allow(unused)]

fn f(s: String) {}

fn take(s: String) {}
fn copy(s: i32) {}

fn main() {

    // owner of rust is s
    let s = String::from("rust");

    // owner of -1 is i
    let i: i32 = -1;

    // owner of rust is s1
    let s1 = s;
    
    // owner of rust is s2
    let s2 = s1;

    // owner of -1 is i 
    let i: i32 = -1;

    // owner of -1 is i1 
    let i1 = i;

    println!("{i} {i1}");

    

    let p = String::from("to drop");
    // 
    // this means {} is like if(true) {}
    // inner scope 
    // means let p = p;
    // drops
    // inside is like p = p;
    // now the alter p is drop;
    //
    {
        p;
    }

    println!("{p}");
    

    let simple = String::from("take ownsership");
    take(simple);
    // now i can print simple because function take ownership
    //
    //this works
    //
    let i: i32 = -1;
    copy(i);
    println!("{i}")

}
