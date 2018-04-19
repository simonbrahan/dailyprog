extern crate primal;

use std::env;

fn get_input() -> usize {
    env::args()
        .nth(1)
        .expect("Single positive integer expected as argument")
        .parse::<usize>()
        .expect("Single positive integer expected as argument")
}

fn list_primes_until(input: usize) -> Vec<usize> {
    if input < 2 {
        return vec![];
    }

    let sieve = primal::Sieve::new(input.clone());
    return sieve.primes_from(0).take_while(|x| x <= &input).collect();
}

fn get_goldbach_solution(input: usize) -> Option<[usize; 3]> {
    for first_prime in list_primes_until(input) {
        for second_prime in list_primes_until(input - first_prime) {
            for third_prime in list_primes_until(input - first_prime - second_prime) {
                if first_prime + second_prime + third_prime == input {
                    return Some([first_prime, second_prime, third_prime]);
                }
            }
        }
    }

    return None;
}

fn main() {
    let input = get_input();

    let solution = get_goldbach_solution(input).expect("No solution found for input");
    println!(
        "{} = {} + {} + {}",
        input,
        solution[0],
        solution[1],
        solution[2]
    );
}
