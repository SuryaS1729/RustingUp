//borrowing and ownership in rust

// fn main() {
//     let mut s1: String = String::from("sahithi");
//     let s2 = &mut s1;
//     s2.push_str(" likes surya");
//     let s3 = &s1;
// }

//structs in rust
//
//

// struct Rect {
//     height: f32,
//     width: f32,
// }

// impl Rect {
//     fn area(&self) -> f32 {
//         return self.width * self.height;
//     }

//     fn perimeter(&self) -> f32 {
//         return 2.0 * (self.width + self.height);
//     }

//     fn print_something(a: u32) {
//         println!("{}, {}", "static function", a) // more like singletons or object class in kotlin and javascript
//     }
// }
// fn main() {
//     let r = Rect {
//         width: 10.0,
//         height: 12.0,
//     };

//     println!("{}, {}", r.width, r.height);
//     println!("{}", r.area());
//     println!("{}", r.perimeter());
//     Rect::print_something(10);
// }

//Enums in Rust

// enum Direction {
//     North,
//     South,
//     East,
//     West
// }
// fn main(){
//     let direction = Direction::South;

//     steer(dir: direction)

// }

// fn steer(dir: Direction){
//     match dir {
//         Direction::East=> print!("East Direction"),
//         Direction::West=> print!("West Direction"),
//         Direction::North=> print!("North Direction"),
//         Direction::South=> print!("South Direction")

//     }
// }
// Enums with values in Rust

// enum Shape {
//     Circle(f32),             // radius
//     Rectangle(f32, f32),     // width, height
//     Triangle(f32, f32, f32), // side1, side2, side3
// }

// fn main() {
//     let shape = Shape::Rectangle(10.0, 12.0);

//     let area_of_shape = area(shape);

//     println!("Area of the shape is: {:?}", area_of_shape);
// }

// fn area(shape: Shape) -> f32 {
//     return match shape {
//         Shape::Circle(radius) => std::f32::consts::PI * radius * radius,
//         Shape::Rectangle(width, height) => width * height,
//         Shape::Triangle(side1, side2, side3) => {
//             let s = (side1 + side2 + side3) / 2.0;
//             (s * (s - side1) * (s - side2) * (s - side3)).sqrt()
//         }
//     };
// }
