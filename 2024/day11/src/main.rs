use std::collections::HashMap;

fn main() {
    let stones: Vec<usize> = std::fs::read_to_string("input.txt")
        .expect("Failed to read input file")
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    part1(stones.clone());
    part2(stones);
}

fn part1(stones: Vec<usize>) {
    println!("Part 1: {}", calc_stones(stones, 25));
}

fn part2(stones: Vec<usize>) {
    println!("Part 2: {}", calc_stones(stones, 75));
}

fn calc_stones(stones: Vec<usize>, iters: usize) -> usize {
    let mut stones_hash_map: HashMap<usize, usize> =
        stones.iter().fold(HashMap::new(), |mut acc, x| {
            *acc.entry(*x).or_insert(0) += 1;
            acc
        });
    for _ in 0..iters {
        let mut new_stones_hash_map: HashMap<usize, usize> = HashMap::new();
        for (number, count) in &stones_hash_map {
            // First rule
            if *number == 0 {
                *new_stones_hash_map.entry(1).or_insert(0) += count;
                continue;
            }
            // Second rule
            let number_of_digits = number.to_string().len();
            if number_of_digits % 2 == 0 {
                let first_half: usize = number.to_string()[0..(number_of_digits / 2)]
                    .parse()
                    .unwrap();
                let second_half: usize = number.to_string()[(number_of_digits / 2)..]
                    .parse()
                    .unwrap();
                *new_stones_hash_map.entry(first_half).or_insert(0) += count;
                *new_stones_hash_map.entry(second_half).or_insert(0) += count;
                continue;
            }
            // Third rule
            *new_stones_hash_map.entry(*number * 2024).or_insert(0) += count;
        }
        stones_hash_map = new_stones_hash_map;
    }
    stones_hash_map.iter().fold(0, |acc, (_, v)| acc + v)
}
