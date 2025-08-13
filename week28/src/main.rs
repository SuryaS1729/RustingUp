// use chrono::{Local, Utc};

// fn main() {
//     let utc = Utc::now();
//     let local = Local::now();
//     print!("{}", utc); // e.g. `2014-11-28T12:45:59.324310806Z`
//     print!("{}", local) // e.g. `2014-11-28T12:45:59.324310806+00:00`
// }
// some packages to look at dotenv, uuid, tui, thiserror, sqlx
//
// Generics in rust
//
// Example 1

// fn main() {}

// // fn sum_u32(a: u32, b: u32) -> u32 {
// //     return a + b;
// // }

// // fn sum_f32(a: f32, b: f32) -> f32 {
// //     return a + b;
// // } code repition ain't it?

// fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
//     return a + b;
// } // we add a trait bound here
//
// Example 2

// fn main() {
//     print_variable(1);
//     print_variable(1.0);
//     print_variable(String::from("hey there"));
// }

// fn print_variable<T: std::fmt::Display>(a: T) {
//     println!("{}", a)
// }// see how we added the display trait here to the generic type to let rust know that this type has this trait
// //so rust allows us to run those operations

//Generics over Structs
