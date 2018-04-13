use std::env;

fn main() {
    let keyword = env::args().nth(1).unwrap();
    let body = env::args().nth(2).unwrap();
    let transpose_table = get_transpose_table(&keyword);

    let mut output = String::new();
    for (input_idx, input_char) in body.char_indices() {
        let transpose_distance = transpose_table[input_idx % transpose_table.len()];
        output.push(alpha_idx_to_char(
            (char_to_alpha_idx(input_char) + transpose_distance) % 26,
        ));
    }

    println!("{}", output);
}

fn get_transpose_table(keyword: &str) -> Vec<usize> {
    return keyword.chars().map(char_to_alpha_idx).collect();
}

fn char_to_alpha_idx(character: char) -> usize {
    return "abcdefghijklmnopqrstuvwxyz"
        .find(character)
        .expect("Bad char for char_to_alpha_idx");
}

fn alpha_idx_to_char(loc: usize) -> char {
    return "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .nth(loc)
        .expect("Bad loc for alpha_idx_to_char");
}
