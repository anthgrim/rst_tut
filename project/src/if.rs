#![allow(unused)]

fn main() {
    let age = 8;

    if (age >= 1) && (age <= 18) {
        println!("Young");
    } else if (age == 21) || (age == 50) {
        println!("Young or Old")
    } else if age >= 65 {
        println!("Old")
    } else {
        println!("Not considered")
    }
}
