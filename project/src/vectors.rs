#![allow(unused)]

fn main() {
    // Vectors
    // Can grow if mutable
    // can only store values of the same type

    // Empty Vector
    let vec1: Vec<i32> = Vec::new();

    // Initialized with values
    let mut vec2: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Adding
    vec2.push(6);

    // Get
    println!("First: {}", vec2[0]);

    // Verify if value exists
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("Second {}", second),
        None => println!("Does not exist"),
    };

    // loop
    for i in &mut vec2 {
        *i *= 2;
    }

    for i in &vec2 {
        println!("{}", i);
    }

    // length
    println!("Length: {}", vec2.len());

    // Pop
    println!("Popped {:?}", vec2.pop());
}
