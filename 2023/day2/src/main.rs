use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Debug)]
struct Game {
    number: usize,
    subsets: Vec<Subset>,
}

#[derive(Clone, Debug)]
struct Subset {
    red: usize,
    green: usize,
    blue: usize,
}

fn main() {
    let file = File::open("input.txt").expect("Could not open input file");
    let reader = io::BufReader::new(file);

    let mut games: Vec<Game> = Vec::new();

    reader.lines().into_iter().for_each(|line| {
        let line = line.expect("Failed to read line");
        let game_and_subsets: Vec<&str> = line.split(": ").collect();
        let game_number: usize = game_and_subsets[0]
            .split_whitespace()
            .collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        let mut subsets = Vec::new();
        game_and_subsets[1]
            .split(";")
            .map(|s| s.trim())
            .for_each(|subset_str| {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                subset_str
                    .split(",")
                    .map(|s| s.trim())
                    .for_each(|color_str| {
                        let color: Vec<&str> = color_str.split_whitespace().collect();
                        let color_number: usize = color[0].parse().unwrap();
                        let color_name = color[1];
                        match color_name {
                            "red" => red = color_number,
                            "green" => green = color_number,
                            "blue" => blue = color_number,
                            _ => (),
                        };
                    });
                subsets.push(Subset { red, green, blue });
            });

        games.push(Game {
            number: game_number,
            subsets,
        });
    });

    part1(games.clone());
    part2(games);
}

fn part1(games: Vec<Game>) {
    let sum: usize = games
        .iter()
        .filter(|game| {
            let max_red = game.subsets.iter().map(|subset| subset.red).max().unwrap();
            let max_green = game
                .subsets
                .iter()
                .map(|subset| subset.green)
                .max()
                .unwrap();
            let max_blue = game.subsets.iter().map(|subset| subset.blue).max().unwrap();
            max_red <= 12 && max_green <= 13 && max_blue <= 14
        })
        .map(|game| game.number)
        .sum();
    println!("Part 1: {}", sum);
}

fn part2(games: Vec<Game>) {
    let sum: usize = games
        .iter()
        .map(|game| {
            let max_red = game.subsets.iter().map(|subset| subset.red).max().unwrap();
            let max_green = game
                .subsets
                .iter()
                .map(|subset| subset.green)
                .max()
                .unwrap();
            let max_blue = game.subsets.iter().map(|subset| subset.blue).max().unwrap();
            max_red * max_green * max_blue
        })
        .sum();
    println!("Part 2: {}", sum);
}
