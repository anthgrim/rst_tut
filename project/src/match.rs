fn main() {
    let age2 = 8;

    match age2 {
        1..=18 => println!("Young"),
        21 | 50 => println!("Young or old"),
        65..=u32::MAX => println!("Old"),
        _ => println!("Not considered"),
    }

    let age = 18;
    let voting_age = 18;

    match age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can vote"),
        _ => println!("You gained voting rights"),
    }
}
