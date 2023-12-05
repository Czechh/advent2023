use crate::utils::read_to_vec;
use regex::Regex;

pub fn run() {
    let file_buffer = read_to_vec("./src/inputs/cinco.txt");
    let input = String::from_utf8(file_buffer).unwrap();
    let number_regex = Regex::new(r"\d+").unwrap();

    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut results1: Vec<i64> = vec![];
    let mut results2: Vec<i64> = vec![];
    let mut seeds: Vec<i64> = vec![];
    let mut seeds2: Vec<i64> = vec![];
    let mut seeds2_ranges: Vec<Vec<i64>> = vec![];
    let mut maps: Vec<Vec<Vec<i64>>> = vec![];

    for (i, part) in parts.iter().enumerate() {
        if i == 0 {
            seeds = number_regex
                .find_iter(part)
                .map(|m| m.as_str().parse::<i64>().unwrap())
                .collect();
        }

        let mut local_map_vec: Vec<Vec<i64>> = vec![];

        for (j, line) in part.lines().enumerate() {
            if j == 0 {
                continue;
            }

            local_map_vec.push(
                number_regex
                    .find_iter(line)
                    .map(|m| m.as_str().parse::<i64>().unwrap())
                    .collect(),
            );
        }

        if local_map_vec.len() > 0 {
            maps.push(local_map_vec);
        }
    }

    for seed in &seeds {
        let mut current = seed.clone();
        for map in &maps {
            for entries in map {
                let destination = entries[0];
                let source = entries[1];
                let range = entries[2];

                if current >= source && current <= &source + &range - 1 {
                    let step = current - source;
                    current = destination + step;
                    break;
                }
            }
        }

        results1.push(current);
    }

    for i in (0..seeds.len()).step_by(2) {
        seeds2_ranges.push(vec![seeds[i].clone(), seeds[i + 1].clone()]);
        let mut j = 0;
        while j < seeds[i + 1].clone() {
            seeds2.push(seeds[i].clone() + j);
            j += 1;
        }
    }

    for seed in &seeds2 {
        let mut current = seed.clone();
        for map in &maps {
            for entries in map {
                let destination = entries[0];
                let source = entries[1];
                let range = entries[2];

                if current >= source && current <= &source + &range - 1 {
                    let step = current - source;
                    current = destination + step;
                    break;
                }
            }
        }

        results2.push(current);
    }

    let min1 = results1.iter().min().unwrap_or(&0);
    let min2 = results2.iter().min().unwrap_or(&0);

    println!("min 1 {:?}", min1);
    println!("min 2 {:?}", min2);
}
