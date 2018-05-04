use std::env;

fn get_input() -> usize {
    env::args()
        .nth(1)
        .expect("Single positive integer expected as argument")
        .parse::<usize>()
        .expect("Single positive integer expected as argument")
}

fn kolakolski_seq(seq_ln: usize) -> Vec<usize> {
    let mut output = vec![1, 2, 2];

    let mut current_iteration = 3;
    while output.len() < seq_ln {
        let examine_digit = output[current_iteration - 1];

        if current_iteration % 2 == 0 {
            output.extend(vec![2; examine_digit]);
        } else {
            output.extend(vec![1; examine_digit]);
        }

        current_iteration += 1;
    }

    return output;
}

fn kolakolski_count(seq_ln: usize) -> [usize; 2] {
    let seq = kolakolski_seq(seq_ln);
    let count_1s = seq.iter().filter(|digit| **digit == 1).count();
    return [count_1s, seq_ln - count_1s];
}

fn main() {
    println!("{:?}", kolakolski_count(get_input()));
}

#[test]
fn test_kolakolski_seq() {
    assert_eq!(vec![1, 2, 2], kolakolski_seq(3));
    assert_eq!(
        vec![1, 2, 2, 1, 1, 2, 1, 2, 2, 1, 2, 2, 1, 1, 2, 1, 1, 2, 2, 1, 2, 1, 1, 2, 1, 2, 2, 1, 1],
        kolakolski_seq(29)
    );
}

#[test]
fn test_kolakolski_count() {
    assert_eq!([5, 5], kolakolski_count(10));
    assert_eq!([49, 51], kolakolski_count(100));
    assert_eq!([502, 498], kolakolski_count(1000));
}
