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
    let mut input = input
        .split_whitespace()
        .map(|s| s.parse().expect("Parse Error"));
    let mut count: u64 = 0;
    let mut largest: u64 = input.next().expect("No input");
    for e in input {
        if e < largest {
            count += largest - e;
        }
        if e > largest {
            largest = e;
        }
    }
    println!("{}", count);
}
