#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn print_sum(x: i32, y: i32) -> () {
    println!("{} + {} = {}", x, y, x + y);
}

fn get_sum(x: i32, y: i32) -> i32 {
    x + y
}

// Can also use return
fn ret_sum(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    print_sum(5, 67);

    let sum_result = get_sum(75, 89);
    println!("{}", sum_result);
}
