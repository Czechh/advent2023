use crate::utils::read_to_vec;
use std::collections::HashMap;

pub fn run() {
    let file_buffer = read_to_vec("./src/inputs/dos.txt");
    let input = String::from_utf8(file_buffer).unwrap();
    let map = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let lines = input.lines();

    let mut sum1 = 0;
    let mut sum2 = 0;

    for (i, line) in lines.enumerate() {
        let parts: Vec<&str> = line.split(": ").collect();
        let games: Vec<&str> = parts[1].split("; ").collect();
        let mut bad_game = false;
        let mut game_set_map = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        for game in games {
            let parts: Vec<&str> = game.split(", ").collect();
            let mut game_map = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

            for color_pairs in parts {
                let color_parts: Vec<&str> = color_pairs.split(" ").collect();
                let color = color_parts[1];
                let num = color_parts[0].parse::<i32>().unwrap();
                let score = game_map.get(color).unwrap() + num;
                game_map.insert(color, score);
            }

            for (color, score) in game_map {
                if map.get(color).unwrap() < &score {
                    bad_game = true;
                }

                let game_set_score = game_set_map.get(color).unwrap();

                if game_set_score < &score {
                    game_set_map.insert(color, score);
                }
            }
        }

        if !bad_game {
            sum1 += i + 1;
        }

        let mut cube_multiplied = 1;

        for (_, score) in game_set_map {
            cube_multiplied *= score;
        }

        sum2 += cube_multiplied;
    }

    println!("Sum 1: {:?}", sum1);
    println!("Sum 2: {:?}", sum2);
}
