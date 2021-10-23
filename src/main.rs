#![allow(non_snake_case)]
#![allow(unused)]

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut arr = User{
        username: String::from("adasda"),
        email:String::from("wesl@come!"),
        sign_in_count: 13,
        active: true,
    };
    let brr = arr.username;
    println!("{}",brr);
}