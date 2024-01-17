use std::collections::HashSet;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    let data: Vec<char> = input.trim().chars().collect();
    generate(data);
}

fn generate(data: Vec<char>) {
    let mut set: HashSet<String> = HashSet::new();
    let current = String::new();
    let mut indexes: HashSet<usize> = HashSet::new();
    for i in 0..data.len() {
        indexes.insert(i);
    }
    recurse(&data, &current, &mut set, &mut indexes);
    println!("{}", set.len());
    let mut ans: Vec<String> = set.into_iter().collect();
    ans.sort();
    for item in ans {
        println!("{}", item);
    }
}

fn recurse(
    data: &Vec<char>,
    current: &String,
    set: &mut HashSet<String>,
    indexes: &mut HashSet<usize>,
) {
    if current.len() >= data.len() {
        set.insert(current.clone());
        return;
    }
    let s: String = current.to_string().clone();
    for &i in indexes.iter() {
        let mut d: String = s.to_string().clone();
        let mut new_indexes = indexes.clone();
        d.push(data[i]);
        new_indexes.remove(&i);
        recurse(data, &d, set, &mut new_indexes)
    }
}
