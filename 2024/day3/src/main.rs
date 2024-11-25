use std::fs::File;
use std::io::{self, BufRead};

use regex::Regex;

fn main() {
    let file = File::open("input.txt").expect("Could not open input file");
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .into_iter()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    part1(lines.clone());
    part2(lines);
}

fn part1(lines: Vec<String>) {
    let re = Regex::new(r"mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\)").unwrap();
    let result: usize = lines
        .iter()
        .map(|line| {
            re.captures_iter(line)
                .map(|cap| {
                    let x = cap.name("x").unwrap().as_str().parse::<usize>().unwrap();
                    let y = cap.name("y").unwrap().as_str().parse::<usize>().unwrap();
                    x * y
                })
                .sum::<usize>()
        })
        .sum();
    println!("Part 1: {}", result);
}

fn part2(lines: Vec<String>) {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut result: usize = 0;
    let mut is_do: bool = true;
    lines.iter().for_each(|line| {
        re.find_iter(line).for_each(|m| {
            let instruction = m.as_str();
            match instruction {
                "do()" => is_do = true,
                "don't()" => is_do = false,
                _ => {
                    if is_do {
                        let multiplied: usize = instruction
                            .replace("mul(", "")
                            .replace(")", "")
                            .split(",")
                            .map(|x| x.parse::<usize>().unwrap())
                            .product();
                        result += multiplied;
                    }
                }
            }
        })
    });
    println!("Part 2: {}", result);
}
