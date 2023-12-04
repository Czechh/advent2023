use crate::utils::read_to_vec;
use regex::Regex;
use std::cmp::min;
use std::collections::HashMap;

fn append_to_gear_map(
    gears: &mut HashMap<(usize, usize), Vec<i32>>,
    i: usize,
    j: usize,
    number: i32,
) {
    if !gears.contains_key(&(i, j)) {
        gears.insert((i, j), vec![number]);
    } else {
        let existing: &mut Vec<i32> = gears.get_mut(&(i, j)).unwrap();
        existing.push(number);
    }
}

pub fn run() {
    let file_buffer = read_to_vec("./src/inputs/tres.txt");
    let input = String::from_utf8(file_buffer).unwrap();
    let matrix = input
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut sum1 = 0;
    let mut sum2 = 0;
    let number_regex = Regex::new(r"\d+").unwrap();
    let mut gears = HashMap::new();

    for i in 0..matrix.len() {
        let line = &matrix[i];
        for number in number_regex.find_iter(line) {
            let mut adjacent_symbol = false;
            let end_index = min(number.end() + 1, line.len());
            // Top
            if i > 0 {
                let line_above = &matrix[i - 1].as_bytes();
                for j in number.start().saturating_sub(1)..end_index {
                    if line_above[j] != b'.' && !line_above[j].is_ascii_alphanumeric() {
                        adjacent_symbol = true;
                        if line_above[j] == b'*' {
                            let parsed_number = number.as_str().parse::<i32>().unwrap();
                            append_to_gear_map(&mut gears, i - 1, j, parsed_number);
                        }
                    }
                }
            }

            //
            if i < matrix.len() - 1 {
                let line_bellow = &matrix[i + 1].as_bytes();
                for j in number.start().saturating_sub(1)..end_index {
                    if line_bellow[j] != b'.' && !line_bellow[j].is_ascii_alphanumeric() {
                        adjacent_symbol = true;
                        if line_bellow[j] == b'*' {
                            let parsed_number = number.as_str().parse::<i32>().unwrap();
                            append_to_gear_map(&mut gears, i + 1, j, parsed_number);
                        }
                    }
                }
            }

            // Left
            if number.start() > 0 {
                let val = &line.as_bytes()[number.start() - 1];
                if *val != b'.' && !val.is_ascii_alphanumeric() {
                    adjacent_symbol = true;
                    if *val == b'*' {
                        let parsed_number = number.as_str().parse::<i32>().unwrap();
                        append_to_gear_map(&mut gears, i, number.start() - 1, parsed_number);
                    }
                }
            }
            // Right
            if number.end() < line.len() {
                let val = &line.as_bytes()[number.end()];
                if *val != b'.' && !val.is_ascii_alphanumeric() {
                    adjacent_symbol = true;
                    if *val == b'*' {
                        let parsed_number = number.as_str().parse::<i32>().unwrap();
                        append_to_gear_map(&mut gears, i, number.end(), parsed_number);
                    }
                }
            }

            if adjacent_symbol {
                sum1 += number.as_str().parse::<i32>().unwrap();
            }
        }
    }

    for (_, value) in gears.into_iter() {
        if value.len() > 1 {
            sum2 += value[0] * value[1];
        }
    }

    println!("Sum 1 {}", sum1);
    println!("Sum 2 {}", sum2);
}
