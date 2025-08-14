// //Macros in Rust,
// // .. used to write dsl's, expands into larger code at compile time

// fn main() {
//     println!("Hello, world!");
//     // this line expands to -> ::std::io::_print(format_args!("Hello, world!\n")); on cargo expand

//     let v = vec![2, 3, 4]; // declarative macros -> replace code written with a different code during compile time
//     print!("{:?}", v)

// }

//writing a macro, declarative

// macro_rules! say_hello {
//     () => {
//         println!("Hello, sahithi")
//     };
// }

// fn main() {
//     say_hello!()
// }

// procedural macros

// procedural macros are used to write custom derive traits, attribute-like macros, and function-like macros
//
//1. Custom derive Macros
// #[derive(Debug)]
// struct User {
//     name: String,
//     age: u32,
// }

// fn main() {
//     let u = User {
//         name: String::from("Sahithi"),
//         age: 23,
//     };

//     // print!("{}", u); // this will not work as User does not implement the Display trait
//     print!("{:?}", u); // this will work as Userdoes not implement the Debug trait, will work when we add #[derive(Debug)]
// }

// 2. Attribute-like Macros

// #[route("GET")] //<- this is a custom attribute-like macro
// fn home() {
//     println!("Welcome to home page")
// }

//annotations work at run time, macros work at compile time

//implementing macros, how to use them, instead of the internals
//debug macro exists, display macro does not exist, we can implement it ourselves
// use std::fmt::Display;

// #[derive(Debug)]
// struct User {
//     name: String,
//     age: u32,
// }

// impl Display for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "User: {}, Age: {}", self.name, self.age)
//     }
// }
// fn main() {
//     let u = User {
//         name: String::from("Sahithi"),
//         age: 23,
//     };

//     print!("{}", u); // this will not work as User does not implement the Display trait
//     print!("{:?}", u); // this will work as Userdoes not implement the Debug trait, will work when we add #[derive(Debug)]
// }

//copy clone macros
//
// fn main() {
//     let user1 = String::from("Sahithi");
//     let user2 = user1;

//     print!("{},{}", user1, user2)//ownership rulesrevision
// }
//
#[derive(Debug, Clone, Copy)]
struct User {
    is_male: bool,
    age: u32,
}
fn main() {
    let u1 = User {
        is_male: true,
        age: 23,
    };

    let u2 = u1; // this will not work as User does not implement the Copy trait, we can implement the Copy trait by adding #[derive(Copy, Clone)] to the struct

    print!("{:?},{:?}", u1, u2)
}
