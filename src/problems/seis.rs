pub fn run() {
    let times = vec![41, 96, 88, 94];
    let final_time = 41968894;
    let distances = vec![214, 1789, 1127, 1055];
    let final_distance: i64 = 214178911271055;
    let mut result1 = 1;
    let mut result2 = 1;

    for (i, time) in times.iter().enumerate() {
        let mut winner_distances = vec![];
        let mut hold_time = 0;

        while hold_time <= *time {
            let remaining_time = time - hold_time;
            let distance_travelled = remaining_time * hold_time;

            if distance_travelled > distances[i] {
                winner_distances.push(distance_travelled);
            }

            hold_time += 1;
        }

        result1 *= winner_distances.len();
    }

    let mut winner_distances = vec![];
    let mut hold_time = 0;

    while hold_time <= final_time {
        let remaining_time = final_time - hold_time;
        let distance_travelled = remaining_time * hold_time;

        if distance_travelled > final_distance {
            winner_distances.push(distance_travelled);
        }

        hold_time += 1;
    }

    result2 *= winner_distances.len();

    println!("Result 1 {}", result1);
    println!("Result 2 {}", result2);
}
