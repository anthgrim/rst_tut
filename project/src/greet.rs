#![allow(unused)]

use std::io;

fn main() {
    println!("What's your name?");
    let mut name: String = String::new();
    let greeting = "Nice to meet you";

    let stdn = io::stdin();
    stdn.read_line(&mut name).expect("Didn't receive input");

    println!("Hello {}! {}", name.trim_end(), greeting);
}
