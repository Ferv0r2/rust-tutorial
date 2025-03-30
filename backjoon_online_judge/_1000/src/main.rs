use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("input error");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let sum: i32 = parts.iter().map(|s| s.parse::<i32>().expect("error")).sum();
    println!("{}", sum);
}
