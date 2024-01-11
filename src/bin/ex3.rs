use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u64 = input.trim().parse().expect("Please type a number!");
    divide_into_equal_sets(input)
}

fn divide_into_equal_sets(n: u64) {
    let mut total_sum: u64 = n * (n + 1) / 2;
    if total_sum % 2 != 0 {
        println!("NO");
        return;
    }
    let mut set1: Vec<u64> = Vec::new();
    let mut set2: Vec<u64> = Vec::new();
    total_sum = total_sum / 2;
    for i in (1..n + 1).rev() {
        if total_sum >= i {
            set1.push(i);
            total_sum -= i;
        } else {
            set2.push(i);
        }
    }
    println!("YES");
    println!("{}", set1.len());
    for i in set1 {
        print!("{} ", i);
    }
    println!("");
    println!("{}", set2.len());
    for i in set2 {
        print!("{} ", i);
    }
}
