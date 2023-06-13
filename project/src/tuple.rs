fn main() {
    let my_tuple: (u8, String, f64) = (47, "John".to_string(), 1.155636997);
    println!("Name: {}", my_tuple.1);

    let (v1, v2, v3) = my_tuple;
    println!("Age: {}", v1);
}
