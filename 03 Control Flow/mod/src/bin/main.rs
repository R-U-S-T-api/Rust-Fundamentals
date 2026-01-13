


#![allow(unused)]


use mods::my;

fn main() {
    my::print();
    my::a::print();

    let s = my::a::S {
        id: 1,
        name: "John".to_string()
    };


    println!("{:?}", s);
    my::call_foo();
    my::a::call_foo();
}
