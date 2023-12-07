use crate::utils::read_to_vec;
use std::cmp::Ordering;
use std::collections::HashMap;

fn get_hand(hand: HashMap<char, i32>) -> i32 {
    let mut has_three = false;
    let mut has_pair = false;

    for (_, value) in hand {
        if value == 5 {
            return 6; // five of a kind
        } else if value == 4 {
            return 5; // four of a kind
        } else if value == 3 {
            has_three = true;
            if has_pair {
                return 4; // full house
            }
        } else if value == 2 {
            if has_pair {
                return 2; // two pairs
            }

            if has_three {
                return 4; // full house
            }
            has_pair = true;
        }
    }

    if has_three {
        return 3; // three of a kind
    }

    if has_pair {
        return 1; // pair
    }

    return 0; // nonde
}

pub fn run() {
    let file_buffer = read_to_vec("./src/inputs/siete.txt");
    let input = String::from_utf8(file_buffer).unwrap();
    let mut sum1 = 0;
    let mut sum2 = 0;

    let card_ranking = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);

    let card_ranking2 = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
    ]);

    let mut results1: Vec<(&str, &str, HashMap<char, i32>, i32)> = input
        .lines()
        .map(|s| {
            let parts: Vec<&str> = s.split(" ").collect();
            let mut map: HashMap<char, i32> = HashMap::new();

            for c in parts[0].chars() {
                *map.entry(c.to_owned()).or_default() += 1;
            }

            let hand = get_hand(map.clone());

            (parts[0], parts[1], map, hand)
        })
        .collect();

    let mut results2: Vec<(&str, &str, HashMap<char, i32>, i32)> = input
        .lines()
        .map(|s| {
            let parts: Vec<&str> = s.split(" ").collect();
            let mut map: HashMap<char, i32> = HashMap::new();
            let mut j = 0;

            for c in parts[0].chars() {
                if c == 'J' {
                    j += 1;
                } else {
                    *map.entry(c.to_owned()).or_default() += 1;
                }
            }

            if j > 0 {
                let o_max_key = { map.iter().max_by_key(|entry| entry.1).map(|(key, _)| *key) };

                if let Some(max_key) = o_max_key {
                    if let Some(val) = map.get_mut(&max_key) {
                        *val += j;
                    }
                } else {
                    map.insert('J', 5);
                }
            }

            let hand = get_hand(map.clone());

            (parts[0], parts[1], map, hand)
        })
        .collect();

    results1.sort_by(|(x, _, _, a), (y, _, _, b)| match a.cmp(&b) {
        Ordering::Equal => {
            let mut order = Ordering::Equal;
            let x_chars: Vec<char> = x.chars().collect();
            let y_chars: Vec<char> = y.chars().collect();
            let mut i = 0;
            while order == Ordering::Equal {
                let first = card_ranking.get(&x_chars[i]).unwrap();
                let second = card_ranking.get(&y_chars[i]).unwrap();
                order = first.cmp(second);
                i += 1;
            }
            order
        }
        other => other,
    });

    results2.sort_by(|(x, _, _, a), (y, _, _, b)| match a.cmp(&b) {
        Ordering::Equal => {
            let mut order = Ordering::Equal;
            let x_chars: Vec<char> = x.chars().collect();
            let y_chars: Vec<char> = y.chars().collect();
            let mut i = 0;
            while order == Ordering::Equal {
                let first = card_ranking2.get(&x_chars[i]).unwrap();
                let second = card_ranking2.get(&y_chars[i]).unwrap();
                order = first.cmp(second);
                i += 1;
            }
            order
        }
        other => other,
    });

    for (i, (_, val, _, _)) in results1.iter().enumerate() {
        let num: i32 = val.parse().unwrap();
        let multiplier = (i as i32 + 1) * num;
        sum1 += multiplier;
    }

    for (i, (_, val, _, _)) in results2.iter().enumerate() {
        let num: i32 = val.parse().unwrap();
        let multiplier = (i as i32 + 1) * num;
        sum2 += multiplier;
    }

    println!("Sum 1 {}", sum1);
    println!("Sum 2 {}", sum2);
}
