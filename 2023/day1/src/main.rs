use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").expect("Could not open input file");
    let reader = io::BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    reader.lines().into_iter().for_each(|line| {
        let line = line.expect("Failed to read line");
        lines.push(line);
    });

    part1(lines.clone());
    part2(lines);
}

fn part1(lines: Vec<String>) {
    let sum: usize = lines
        .iter()
        .map(|line| {
            let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
            let mut number = String::new();
            number.push(digits[0]);
            number.push(digits[digits.len() - 1]);
            number.parse::<usize>().unwrap()
        })
        .sum();
    println!("Part 1: {}", sum);
}

fn part2(lines: Vec<String>) {
    let sum: usize = lines
        .iter()
        .map(|line| {
            let mut digits: Vec<usize> = Vec::new();
            line.chars().into_iter().enumerate().for_each(|(i, c)| {
                if c.is_digit(10) {
                    digits.push(c.to_digit(10).unwrap() as usize);
                }
                if line[i..].starts_with("one") {
                    digits.push(1);
                }
                if line[i..].starts_with("two") {
                    digits.push(2);
                }
                if line[i..].starts_with("three") {
                    digits.push(3);
                }
                if line[i..].starts_with("four") {
                    digits.push(4);
                }
                if line[i..].starts_with("five") {
                    digits.push(5);
                }
                if line[i..].starts_with("six") {
                    digits.push(6);
                }
                if line[i..].starts_with("seven") {
                    digits.push(7);
                }
                if line[i..].starts_with("eight") {
                    digits.push(8);
                }
                if line[i..].starts_with("nine") {
                    digits.push(9);
                }
            });
            let mut number = String::new();
            number.push_str(&digits[0].to_string());
            number.push_str(&digits[digits.len() - 1].to_string());
            number.parse::<usize>().unwrap()
        })
        .sum();
    println!("Part 2: {}", sum);
}
