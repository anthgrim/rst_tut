#![allow(unused)]

// Stack stores values in a last in first out format
// Data on the stack must have a defined fixed size

// Heap When putting data on the heap you request a certain amount of space.
// The OS finds space available and returns an address for that space called a pointer

// Ownership rules
// 1. Each value has a variable that's called its ownwer
// 2. There is only one owner at a time
// 3. When the owner goes out of scope the value disappears

fn print_str(x: String) -> () {
    println!("{x}");
}

fn print_return(x: String) -> String {
    println!("{x}");
    x
}

fn mod_string(x: &mut String) -> () {
    x.push_str(" - mod");
    println!("Done Mode: {x}");
}

fn main() {
    let str1 = String::from("world");
    let str2 = str1; // str1 doesn't exist anymore it was transferred to str2
    let str3 = str2.clone(); // Makes a copy, str2 still exists

    print_str(String::from("First String"));
    let mut returned = print_return(String::from("Second String"));

    mod_string(&mut returned);
    print_str(returned);
}
