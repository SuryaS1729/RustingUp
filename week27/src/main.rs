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

struct Rect {
    height: f32,
    width: f32,
}

impl Rect {
    fn area(&self) -> f32 {
        return self.width * self.height;
    }

    fn print_something(a: u32) {
        println!("{}, {}", "static function", a) // more like singletons or object class in kotlin and javascript
    }
}
fn main() {
    let r = Rect {
        width: 10.0,
        height: 12.0,
    };

    println!("{}, {}", r.width, r.height);
    println!("{}", r.area());
    Rect::print_something(10);
}
