#![allow(unused)]

use std::collections::HashMap;

fn main() {
    // Hashmaps
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clerk Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("Flash", "Barry Allen");

    // Iteration
    for (k, v) in heroes.iter() {
        println!("{k} = {v}");
    }

    // Length
    println!("Length {}", heroes.len());

    // Check for specific key
    if heroes.contains_key("Batman") {
        let batman = heroes.get(&"Batman");

        match batman {
            Some(x) => println!("Hero"),
            None => println!("Not a hero"),
        }
    }
}
