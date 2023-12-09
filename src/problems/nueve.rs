use crate::utils::read_to_vec;
use regex::Regex;

pub fn get_next_sequence(sequence: &Vec<i64>) -> Vec<i64> {
    let mut next_sequence: Vec<i64> = vec![];
    for i in 0..sequence.len() - 1 {
        let diff = sequence[i + 1] - sequence[i];
        next_sequence.push(diff)
    }

    next_sequence
}

pub fn is_all_zeros(sequence: &Vec<i64>) -> bool {
    sequence
        .iter()
        .filter(|v| **v == 0)
        .collect::<Vec<_>>()
        .len()
        == sequence.len()
}

pub fn get_next_value_in_sequence(sequences: &Vec<Vec<i64>>) -> i64 {
    let mut new_sequences = sequences.clone();
    for i in (0..sequences.len() - 1).rev() {
        let last = new_sequences[i].len() - 1;
        let last_prev = new_sequences[i + 1].len() - 1;
        let next_value = new_sequences[i][last] + new_sequences[i + 1][last_prev];
        new_sequences[i].push(next_value);
    }

    new_sequences[0][new_sequences[0].len() - 1]
}

pub fn get_prev_value_in_sequence(sequences: &Vec<Vec<i64>>) -> i64 {
    let mut new_sequences = sequences.clone();
    for i in (0..sequences.len() - 1).rev() {
        let prev_value = new_sequences[i][0] - new_sequences[i + 1][0];
        new_sequences[i].insert(0, prev_value);
    }

    new_sequences[0][0]
}

pub fn run() {
    let file_buffer = read_to_vec("./src/inputs/nueve.txt");
    let input = String::from_utf8(file_buffer).unwrap();
    let num_regex = Regex::new(r"(-?)\d+").unwrap();
    let mut result1 = 0;
    let mut result2 = 0;

    for line in input.lines() {
        let mut sub_sequences: Vec<Vec<i64>> = vec![];
        let sequence: Vec<i64> = num_regex
            .find_iter(line)
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();

        sub_sequences.push(sequence.clone());
        let mut current_sub_sequence = sequence.clone();

        while !is_all_zeros(&current_sub_sequence) {
            current_sub_sequence = get_next_sequence(&current_sub_sequence);
            sub_sequences.push(current_sub_sequence.clone());
        }
        result1 += get_next_value_in_sequence(&sub_sequences);
        result2 += get_prev_value_in_sequence(&sub_sequences);
    }

    println!("Part 1 {:?}", result1);
    println!("Part 2 {:?}", result2);
}
