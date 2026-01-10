

```rust


#![allow(unused)]
enum Animal {
    Cat,
    Dog,
    Duck,
    Mouse
}

fn main() {
    
    let x = 6;

    // old way
    // if x == 1 {
    //  println!("{}");
    // } else if x == 3{
    //  println!("{}");
    // } else {
    //
    // }

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other")
    }
    
    match x {
        1 | 2 | 3  => {
            println!("this is multiple match")
        }
        _ => {
            println!("this is multple match on others")
        }
    }

    // to so what x is match
    // use i @
    // if else got blanc 
    match x {
        i @ 1..=10 => println!("between 1 and 10 {i}"),
        _ => println!("between 0 or 11 up")
    }

    
    let animal = Animal::Cat;

    let animal_sound = match animal {
        Animal::Cat => "Meow",
        Animal::Dog => "Woof",
        Animal::Duck => "Quak",
        Animal::Mouse => "Snitch",
        _ => "_"
    };

    println!("animal sound of is {animal_sound}");
    
    let z = Option<i32> = Some(1);

    match x {
        Some(v) => println!("Some {v}"),
        None => println!("none")
    }

    let res: Result<u32, String> = Ok(10);

    match res {
        Ok(val) => println!("Ok {val}"),
        Err(msg) => println!("Err {msg}")
    }


}

```