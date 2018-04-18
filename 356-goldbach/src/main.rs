extern crate primal;

use std::env;

fn get_input() -> usize {
    env::args()
        .nth(1)
        .expect("Single positive integer expected as argument")
        .parse::<usize>()
        .expect("Single positive integer expected as argument")
}

fn list_primes_less_than(input: &usize) -> Vec<usize> {
    let sieve = primal::Sieve::new(input.clone());
    return sieve.primes_from(0).take_while(|x| x < input).collect();
}

fn main() {
    let input = get_input();

    println!("{:?}", list_primes_less_than(&input));
}
