//borrowing and ownership in rust

fn main() {
    let mut s1: String = String::from("sahithi");
    // s1.push_str("loves surya"); //-> here you cannot mutate because, variables are immutable by default
    let s2: &mut String = &mut s1;
    s2.push_str(" loves surya");

    println!("{}", s2)
}
