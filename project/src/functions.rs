#![allow(unused)]

fn say_hi() -> () {
    println!("Hi")
}

fn print_sum(x: i32, y: i32) -> () {
    println!("{} + {} = {}", x, y, x + y);
}

fn get_sum(x: i32, y: i32) -> i32 {
    x + y
}

// Can also use return
fn ret_sum(x: i32, y: i32) -> i32 {
    return x + y;
}

fn get_tuple(x: i32) -> (i32, i32) {
    (x + 1, x + 2)
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
    sum
}

fn main() {
    print_sum(5, 67);

    let sum_result = get_sum(75, 89);
    println!("{}", sum_result);

    let (val1, val2) = get_tuple(5);
    println!("Nums: {val1} {val2}");

    let list = vec![1, 2, 3, 4, 5, 6];
    let sum_of_list = sum_list(&list);
}
