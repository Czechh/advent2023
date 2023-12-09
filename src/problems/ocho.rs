use crate::utils::read_to_vec;
use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;

pub fn run() {
    let file_buffer = read_to_vec("./src/inputs/ocho.txt");
    let input = String::from_utf8(file_buffer).unwrap();
    let word_regex = Regex::new(r"\w+").unwrap();
    let parts: Vec<&str> = input.split("\n\n").collect();
    let directions: Vec<i32> = parts[0]
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect();

    let mut map = HashMap::new();

    for line in parts[1].lines() {
        let arr: Vec<&str> = word_regex.find_iter(line).map(|m| m.as_str()).collect();
        map.insert(arr[0], vec![arr[1], arr[2]]);
    }

    // Part 1
    let mut i = 0;
    let mut found = false;
    let mut current = "AAA";
    while !found {
        let d = i % directions.len();
        let direction = directions[d];
        let next_steps = map.get(current).unwrap();

        current = next_steps[direction as usize];
        i += 1;

        if current == "ZZZ" {
            found = true;
        }
    }

    let currents: Vec<&str> = map
        .iter()
        .map(|(k, _)| *k)
        .filter(|k| k.ends_with("A"))
        .collect();

    let mut periods: Vec<i128> = vec![];

    for c in currents {
        let mut j = 0;
        let mut found = false;
        let mut current = c.clone();

        while !found {
            let d = j % directions.len();
            let direction = directions[d];
            let next_steps = map.get(current).unwrap();

            current = next_steps[direction as usize];
            j += 1;

            if current.ends_with("Z") {
                found = true;
            }
        }

        periods.push(j as i128);
    }

    let mut result2: i128 = 1;
    for p in &periods {
        result2 = lcm(*p, result2);
    }

    println!("Part 1 {}", i);
    println!("Part 2 {:?}", result2);
}
