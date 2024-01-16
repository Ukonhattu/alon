use std::io;

fn main() {
    let mut count = String::new();
    let mut integers = String::new();
    io::stdin()
        .read_line(&mut count)
        .expect("Unable to read count");
    io::stdin()
        .read_line(&mut integers)
        .expect("Unable to read integers");
    let input = integers
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Parse error"))
        .collect();
    collect(input);
}

fn collect(input: Vec<u32>) {
    let mut count: u32 = 0;
    let mut last: u32 = 0;
    for &item in input.iter() {
        if item < last {
            count += 1;
        }
        last = item;
    }
    println!("{}", count);
}
