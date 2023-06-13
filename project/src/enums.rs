#![allow(unused)]

fn main() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;

    // Casting
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    // Enums
    enum Simple {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    // Enums with functions
    impl Simple {
        fn is_weekend(&self) -> bool {
            match self {
                Simple::Saturday | Simple::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Simple = Simple::Monday;
    match today {
        Simple::Monday => println!("Every hates monday"),
        Simple::Tuesday => println!("Tuesday"),
        Simple::Wednesday => println!("Ok"),
        Simple::Thursday => println!("almost"),
        Simple::Friday => println!("Friyay!"),
        Simple::Saturday => println!("Party"),
        Simple::Sunday => println!("Oh God"),
    }

    println!("Is today a weekend? {}", today.is_weekend());
}
