use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").expect("Could not open input file");
    let reader = io::BufReader::new(file);

    let mut equations: Vec<(usize, Vec<usize>)> = Vec::new();

    reader.lines().into_iter().for_each(|line| {
        let line = line.expect("Failed to read line");
        let mut split = line.split(":").map(|x| x.trim());
        let result = split.next().unwrap().parse::<usize>().unwrap();
        let numbers = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        equations.push((result, numbers));
    });

    part1(equations.clone());
    part2(equations);
}

fn part1(equations: Vec<(usize, Vec<usize>)>) {
    let sum: usize = equations
        .iter()
        .filter(|(result, numbers)| {
            let possible_operators = Vec::from(['+', '*']);
            let possible_operator_configurations =
                std::iter::repeat_n(possible_operators, numbers.len())
                    .multi_cartesian_product()
                    .collect::<Vec<_>>();
            possible_operator_configurations.iter().any(|operators| {
                let mut iteration_result = numbers[0];
                for (operator, number) in operators.iter().zip(numbers.iter().skip(1)) {
                    match operator {
                        '+' => iteration_result += number,
                        '*' => iteration_result *= number,
                        _ => panic!("Invalid operator"),
                    }
                }
                iteration_result == *result
            })
        })
        .map(|(result, _)| result)
        .sum();
    println!("Part 1: {}", sum);
}

fn part2(equations: Vec<(usize, Vec<usize>)>) {
    let sum: usize = equations
        .iter()
        .filter(|(result, numbers)| {
            let possible_operators = Vec::from(["+", "*", "||"]);
            let possible_operator_configurations =
                std::iter::repeat_n(possible_operators, numbers.len())
                    .multi_cartesian_product()
                    .collect::<Vec<_>>();
            possible_operator_configurations.iter().any(|operators| {
                let mut iteration_result = numbers[0];
                for (operator, number) in operators.iter().zip(numbers.iter().skip(1)) {
                    match operator {
                        &"+" => iteration_result += number,
                        &"*" => iteration_result *= number,
                        &"||" => {
                            let mut a = iteration_result.to_string();
                            a.push_str(number.to_string().as_str());
                            iteration_result = a.parse::<usize>().unwrap();
                        }
                        _ => panic!("Invalid operator"),
                    }
                }
                iteration_result == *result
            })
        })
        .map(|(result, _)| result)
        .sum();
    println!("Part 2: {}", sum);
}
