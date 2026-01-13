
        #[derive(Debug)]
        pub struct S {
            pub id: u32,
            pub name: String
        }

        pub fn print() {
            println!("my mod -> a mod -> print() ");
        }
        use super::super::foo;
        pub fn call_foo() {
            foo::print();
        }



