use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").expect("Could not open input file");
    let reader = io::BufReader::new(file);

    let equations: Vec<(usize, Vec<usize>)> = reader
        .lines()
        .into_iter()
        .map(|line| {
            let line = line.expect("Failed to read line");
            let mut split = line.split(":").map(|x| x.trim());
            let result = split.next().unwrap().parse::<usize>().unwrap();
            let numbers = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            (result, numbers)
        })
        .collect();

    part1(equations.clone());
    part2(equations);
}

fn part1(equations: Vec<(usize, Vec<usize>)>) {
    let sum: usize = equations
        .iter()
        .filter(|(result, numbers)| is_valid(&numbers[1..].to_vec(), numbers[0], *result))
        .map(|(result, _)| result)
        .sum();
    println!("Part 1: {}", sum);
}

fn part2(equations: Vec<(usize, Vec<usize>)>) {
    let sum: usize = equations
        .iter()
        .filter(|(result, numbers)| is_valid_p2(&numbers[1..].to_vec(), numbers[0], *result))
        .map(|(result, _)| result)
        .sum();
    println!("Part 2: {}", sum);
}

fn is_valid(numbers: &Vec<usize>, result_candidate: usize, result: usize) -> bool {
    if numbers.is_empty() || result_candidate > result {
        return result_candidate == result;
    }

    is_valid(
        &numbers[1..].to_vec(),
        numbers[0] + result_candidate,
        result,
    ) || is_valid(
        &numbers[1..].to_vec(),
        numbers[0] * result_candidate,
        result,
    )
}

fn is_valid_p2(numbers: &Vec<usize>, result_candidate: usize, result: usize) -> bool {
    if numbers.is_empty() || result_candidate > result {
        return result_candidate == result;
    }

    let mut a = result_candidate.to_string();
    a.push_str(numbers[0].to_string().as_str());

    is_valid_p2(
        &numbers[1..].to_vec(),
        numbers[0] + result_candidate,
        result,
    ) || is_valid_p2(
        &numbers[1..].to_vec(),
        numbers[0] * result_candidate,
        result,
    ) || is_valid_p2(&numbers[1..].to_vec(), a.parse::<usize>().unwrap(), result)
}
