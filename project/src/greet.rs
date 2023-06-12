#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    println!("What's your name?");
    let mut name: String = String::new();
    let greeting = "Nice to meet you";

    let stdn = io::stdin();
    stdn.read_line(&mut name).expect("Didn't receive input");

    println!("Hello {}! {}", name.trim_end(), greeting);
}
