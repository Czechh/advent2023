use crate::utils::read_to_vec;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let file_buffer = read_to_vec("./src/inputs/cuatro.txt");
    let input = String::from_utf8(file_buffer).unwrap();
    let number_regex = Regex::new(r"\d+").unwrap();
    let mut sum1 = 0;
    let mut card_copies = input
        .clone()
        .lines()
        .enumerate()
        .map(|(i, _)| (i, 0_i32))
        .collect::<HashMap<_, _>>();

    for (i, card) in input.lines().enumerate() {
        let parts: Vec<&str> = card.split(":").collect();
        let num_parts: Vec<&str> = parts[1].split("|").collect();
        let winning_nums_arr: Vec<String> = number_regex
            .find_iter(num_parts[0])
            .map(|m| m.as_str().to_string())
            .collect();

        let winning_nums: HashSet<String> = winning_nums_arr.into_iter().collect();
        let mut card_score = 0;
        let mut num_matches = 0;

        for number in number_regex.find_iter(num_parts[1]) {
            if winning_nums.contains(number.as_str()) {
                num_matches += 1;
                if card_score == 0 {
                    card_score += 1;
                } else {
                    card_score *= 2;
                }
            }
        }

        sum1 += card_score;

        let additional_copies = card_copies.get(&i).unwrap();
        let end = i + num_matches;
        let mut j = i + 1;
        let copy_increment = 1 + additional_copies;

        while j <= end {
            let copy_num = card_copies.get_mut(&j).unwrap();
            *copy_num += copy_increment;
            j += 1;
        }
    }

    let mut sum2 = input.lines().count() as i32;
    for (_, value) in card_copies.iter() {
        sum2 += value;
    }

    println!("Sum 1 {}", sum1);
    println!("Sum 2 {}", sum2);
}
