use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").expect("Could not open input file");
    let reader = io::BufReader::new(file);

    let mut scratchcards: Vec<String> = Vec::new();

    reader.lines().into_iter().for_each(|line| {
        let line = line.expect("Failed to read line");
        scratchcards.push(line);
    });

    part1(scratchcards.clone());
    part2(scratchcards);
}

fn part1(scratchcards: Vec<String>) {
    let mut scratchcard_winning_counts: HashMap<String, usize> = HashMap::new();
    let sum: usize = scratchcards
        .iter()
        .map(|scratchcard| card_score(scratchcard, &mut scratchcard_winning_counts))
        .sum();
    println!("Part 1: {}", sum);
}

fn part2(scratchcards: Vec<String>) {
    let mut scratchcard_winning_counts: HashMap<String, usize> = HashMap::new();
    let sum: usize = scratchcards
        .iter()
        .enumerate()
        .map(|(card_nr, scratchcard)| {
            count_cards(
                &scratchcards,
                scratchcard,
                card_nr,
                &mut scratchcard_winning_counts,
            )
        })
        .sum();
    println!("Part 2: {}", sum);
}

fn count_cards(
    scratchcards: &Vec<String>,
    scratchcard: &String,
    card_nr: usize,
    scratchcard_winning_counts: &mut HashMap<String, usize>,
) -> usize {
    let mut sum = 1;
    let winning_count = winning_numbers_count(scratchcard, scratchcard_winning_counts);
    for i in (card_nr + 1)..=(card_nr + winning_count) {
        sum += count_cards(
            scratchcards,
            &scratchcards[i],
            i,
            scratchcard_winning_counts,
        );
    }
    sum
}

fn card_score(
    scratchcard: &String,
    scratchcard_winning_counts: &mut HashMap<String, usize>,
) -> usize {
    let winning_count = winning_numbers_count(scratchcard, scratchcard_winning_counts);
    if winning_count == 0 {
        return 0;
    }
    2usize.pow(winning_count as u32 - 1)
}

fn winning_numbers_count(
    scratchcard: &String,
    scratchcard_winning_counts: &mut HashMap<String, usize>,
) -> usize {
    if scratchcard_winning_counts.contains_key(scratchcard) {
        return *scratchcard_winning_counts.get(scratchcard).unwrap();
    }
    let numbers_str = scratchcard
        .split(":")
        .map(|s| s.trim())
        .collect::<Vec<&str>>()[1];
    let numbers = numbers_str
        .split("|")
        .map(|s| s.trim())
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let winning_count = numbers[0].iter().filter(|n| numbers[1].contains(n)).count();
    scratchcard_winning_counts.insert(scratchcard.clone(), winning_count);
    winning_count
}
