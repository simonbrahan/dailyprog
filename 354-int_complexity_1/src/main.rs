use std::env;

fn get_input() -> usize {
    env::args()
        .nth(1)
        .expect("Single positive integer expected as argument")
        .parse::<usize>()
        .expect("Single positive integer expected as argument")
}

fn get_highest_candidate(input: usize) -> usize {
    (input as f64).sqrt().ceil() as usize
}

fn main() {
    let input = get_input();
    let result = (1..get_highest_candidate(input))
        .rev()
        .filter(|num| input % num == 0)
        .nth(0)
        .expect("Wat");

    println!("{} => {}", input, result + input / result);
}
