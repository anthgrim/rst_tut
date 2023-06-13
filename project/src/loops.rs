#![allow(unused)]

fn main() {
    let array_1 = [1, 2, 3, 4];
    println!("first {}", array_1[0]);
    println!("Length {}", array_1.len());

    // loops

    // loop
    let mut loop_idx = 0;
    loop {
        if array_1[loop_idx] == 4 {
            break;
        }

        if array_1[loop_idx] % 2 == 0 {
            println!("Even");
            loop_idx += 1;
            continue;
        }

        loop_idx += 1;
        println!("Odd");
    }

    // while
    let mut idx = 0;
    while idx < array_1.len() {
        println!("Arr: {}", array_1[idx]);
        idx += 1;
    }

    // For loops
    for item in array_1.iter() {
        println!("Value {}", item);
    }
}
