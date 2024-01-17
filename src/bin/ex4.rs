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
        .map(|s| s.parse::<usize>().expect("Parse error"))
        .collect();
    collect(input);
}

fn collect(input: Vec<usize>) {
    let mut count: u32 = 0;
    let mut lookup: Vec<bool> = vec![false; input.len() + 1];
    for c in input {
        if !lookup[c - 1] {
            count += 1;
        }
        lookup[c] = true;
    }
    println!("{}", count);
}
