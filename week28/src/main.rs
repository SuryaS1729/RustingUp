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

// //Generics over Structs
// #[derive(Clone, Copy)]
// struct Rect<T> {
//     width: T,
//     height: T,
// }

// impl<T: std::ops::Mul<Output = T> + Copy> Rect<T> {
//     fn area(&self) -> T {
//         return self.width * self.height;
//     }
// }
// fn main() {
//     let r1 = Rect {
//         width: 10.0,
//         height: 102.0,
//     };

//     let area = r1.area();
//     println!("{}", area)
// }

//generics over enums
// enum Option<T>{
// Some(T)
// None
// }

//traits
// traits are similar to interfaces in ts

use std::f32::consts::PI;

//implement a trait over a struct
//
trait Shape {
    fn area(&self) -> f32;
}
struct Rect {
    width: f32,
    height: f32,
}
struct Circle {
    radius: f32,
}

impl Shape for Rect {
    fn area(&self) -> f32 {
        return self.width * self.height;
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        return PI * self.radius * self.radius;
    }
}

fn print_area_of_shape<T: Shape>(s: T) {
    //the generic is bound to the trait Shape
    println!("{}", s.area())
}
fn main() {
    let r = Rect {
        width: 20.0,
        height: 21.0,
    };
    let c = Circle { radius: 2.0 };

    print_area_of_shape(c);
    print_area_of_shape(r);
}
