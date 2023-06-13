#![allow(unused)]

fn main() {
    // String
    let mut st1 = String::new();

    st1.push('A');
    st1.push_str("Word");

    for word in st1.split_whitespace() {
        println!("{}", word);
    }

    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    // Vector
    let st3 = String::from("x r t h b s a q e t x d j i gg s s h");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();

    for char in v1 {
        println!("{}", char);
    }

    // String literal
    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);

    // bytes
    let byte1 = st5.as_bytes();

    // Slice
    let st6 = &st5[0..6];
    println!("Length {}", st6.len());

    // Delete
    st5.clear();

    // Combine
    let st7 = String::from("Just Some");
    let st8 = String::from("Words");

    let st9 = st7 + &st8;

    for char in st9.bytes() {
        println!("{}", char)
    }
}
