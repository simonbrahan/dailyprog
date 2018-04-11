use std::env;

fn main() {
    let keyword = env::args().nth(1).unwrap();
    let body = env::args().nth(2).unwrap();
    println!("{}", get_idx_of_char(body.chars().next().unwrap()));
    println!("{}", get_char_at_idx(2));
}

fn get_idx_of_char(character: char) -> usize {
    return "abcdefghijklmnopqrstuvwxyz".find(character).unwrap();
}

fn get_char_at_idx(loc: usize) -> char {
    return "abcdefghijklmnopqrstuvwxyz".chars().nth(loc).unwrap();
}
