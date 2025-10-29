// The following code is an example of how to use the serde library for serialization and deserialization.
// It is currently commented out.

// // Import the `Deserialize` and `Serialize` traits from the `serde` library.
// use ::serde::{Deserialize, Serialize};

// // The `#[derive]` attribute automatically implements the specified traits for the struct.
// // - `Serialize`: Allows the struct to be converted into a serialized format (e.g., JSON).
// // - `Deserialize`: Allows the struct to be created from a serialized format.
// // - `Debug`: Allows the struct to be printed for debugging purposes using `{:?}`.
// #[derive(Serialize, Deserialize, Debug)]
// struct User {
//     name: String,
//     age: u8,
// }

// // The main function for the serde example.
// fn main() {
//     // Create an instance of the `User` struct.
//     let user = User {
//         name: String::from("Alice"),
//         age: 30,
//     };

//     // Serialize the `user` instance into a JSON string.
//     // `serde_json::to_string` returns a `Result` which will be `Ok(json_string)` on success
//     // or `Err(error)` on failure.
//     let serialized = serde_json::to_string(&user);
//     match serialized {
//         Ok(json) => println!("Serialized: {}", json),
//         Err(e) => eprintln!("Serialization error: {}", e),
//     }

//     // The following line for deserialization is incorrect and would cause a compile error.
//     // It should take a string slice as an argument.
//     // For example:
//     // let json_data = r#"{"name":"Bob","age":25}"#;
//     // let deserialized: User = serde_json::from_str(json_data).unwrap();
//     // println!("Deserialized: {:?}", deserialized);
//     let deserialized = serde_json::from_str::serialized;
// }

// This is the current main function that will be executed.
//
//

// use borsh::{BorshDeserialize, BorshSerialize};

// #[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]

// struct User {
//     username: String,
//     password: String,
// }

// fn main() {
//     let u: User = User {
//         username: String::from("Sahithi"),
//         password: String::from("123123"),
//     };

//     let mut v: Vec<u8> = Vec::new();

//     let ans = u.serialize(&mut v);

//     match ans {
//         Ok(_) => print!("{:?}", v),
//         Err(_) => print!("Error occurred during serialization"),
//     }
//     let user = User::try_from_slice(&v).unwrap();

//     print!("{}", user.username)
// }

//lifetimes

// fn main() {
//     let str1 = String::from("surya saaket");
//     let ans;
//     {
//         let str2 = String::from("hello world");
//         ans = longest_string(&str1, &str2);
//     }

//     println!("{}", ans)
// }

// fn longest_string<'a, 'b>(s1: &'a String, s2: &'a String) -> &'a String {
//     if s1.len() > s2.len() {
//         return &s1;
//     } else {
//         return &s2;
//     }
// }
//
//
fn choose<'a, 'b>(a: &'a str, b: &'a str, use_a: bool) -> &'a str {
    if use_a { a } else { b }
}

fn main() {
    let string1 = String::from("Hello, world!");
    let string2 = String::from("Hi!");

    let result = choose(&string1, &string2, true);
    println!("Chosen string: {}", result);
}
