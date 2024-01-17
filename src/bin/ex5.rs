use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    let data = Vec::from(input);
}

fn generate(data: Vec<u8>) {}
