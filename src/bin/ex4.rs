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
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Parse error"))
        .collect();
    collect(input);
}

fn collect(mut input: Vec<u32>) {
    let mut count = 0;
    let mut current;
    let mut to_remove = Vec::new();
    while input.is_empty() {
        current = 0;
        for i in 0..input.len() {
            if input[i] >= current {
                to_remove.push(i);
                current = input[i];
            }
        }
        to_remove.iter().for_each(|s| {
            input.remove(*s);
        });
        to_remove.clear();
        count = count + 1;
    }
    println!("{}", count)
}
