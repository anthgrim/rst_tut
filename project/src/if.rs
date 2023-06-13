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

    let mut my_age = 47;
    let can_vote = if my_age >= 18 { true } else { false };

    println!("Can vote: {}", can_vote);
}
