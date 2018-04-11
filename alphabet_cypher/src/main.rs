use std::env;

fn main() {
    let keyword = env::args().nth(1).unwrap();
    let body = env::args().nth(2).unwrap();

    println!("{}, {}", keyword, body);
}
