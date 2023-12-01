use crate::utils::read_to_vec;
use std::collections::HashMap;

pub fn run() {
    let file_buffer = read_to_vec("./src/inputs/uno.txt");
    let input = String::from_utf8(file_buffer).unwrap();

    let sum1 = input
        .lines()
        .map(|line| {
            let mut f_digit: Option<char> = None;
            let mut s_digit = '0';

            for c in line.chars().into_iter() {
                let char_to_number = c.to_digit(10);

                if char_to_number.is_some() {
                    if f_digit.is_none() {
                        f_digit = Some(c);
                    }
                    s_digit = c;
                }
            }
            let full_number = f_digit.unwrap().to_string() + &s_digit.to_string();
            full_number.parse::<i32>().unwrap()
        })
        .sum::<i32>();

    let map = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let sum2 = input
        .lines()
        .map(|line| {
            let mut current_index = 0;
            let mut f_digit: Option<char> = None;
            let mut s_digit = '0';
            for (i, c) in line.char_indices() {
                let char_to_number = c.to_digit(10);

                if char_to_number.is_some() {
                    if f_digit.is_none() {
                        f_digit = Some(c);
                    }
                    s_digit = c;
                    current_index = i + 1;
                } else {
                    let mut temp = current_index;

                    while temp < i {
                        let sub_string = &line[temp..i + 1];

                        if map.contains_key(sub_string) {
                            let c = map.get(sub_string).unwrap();
                            if f_digit.is_none() {
                                f_digit = Some(c.to_owned());
                            }
                            s_digit = c.to_owned();
                            current_index = current_index + 1;
                            break;
                        }
                        temp += 1;
                    }
                }
            }

            let full_number = f_digit.unwrap().to_string() + &s_digit.to_string();
            full_number.parse::<i32>().unwrap()
        })
        .sum::<i32>();

    println!("Sum 1: {:?}", sum1);
    println!("Sum 2: {:?}", sum2);
}
