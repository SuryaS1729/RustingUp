//borrowing and ownership in rust

fn main() {
    let mut s1: String = String::from("sahithi");
    // s1.push_str("loves surya"); //-> here you cannot mutate because, variables are immutable by default
    let s2: &mut String = &mut s1;
    //once you have a mutable reference, you cannot have other references, immutable or mutable to the variable.
    let s3: &mut String = &mut s1;
    s2.push_str(" loves surya");

    s3.push_str("  won't marry surya"); // -> yay, thank god rust does'nt allow this to compile

    println!("{}", s2)
}
