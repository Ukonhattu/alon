use std::io;

fn main() {
    let mut size = String::new();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read line");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let _size: u32 = size.trim().parse().expect("Please type a number!");
    let _input: Vec<u32> = input
        .split(',')
        .map(|s| s.parse::<u32>().expect("parse error"))
        .collect();
}
