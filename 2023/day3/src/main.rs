use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").expect("Could not open input file");
    let reader = io::BufReader::new(file);

    let mut schematic: Vec<Vec<char>> = Vec::new();

    reader.lines().into_iter().for_each(|line| {
        let line = line.expect("Failed to read line");
        let line_chars: Vec<char> = line.chars().collect();
        schematic.push(line_chars);
    });

    part1(schematic.clone());
    part2(schematic);
}

fn part1(schematic: Vec<Vec<char>>) {
    let mut sum = 0;
    for (y, _) in schematic.iter().enumerate() {
        for (x, _) in schematic[y].iter().enumerate() {
            if schematic[y][x].is_digit(10) || schematic[y][x] == '.' {
                continue;
            }
            let mut numbers: Vec<usize> = Vec::new();
            for a in -1..=1 {
                for b in -1..=1 {
                    let checked_x = x as isize + a;
                    let checked_y = y as isize + b;
                    let c = get_element_or_default(&schematic, checked_x, checked_y);
                    if c.is_digit(10) {
                        let number = get_number_digit_is_part_of(
                            &schematic,
                            checked_x as usize,
                            checked_y as usize,
                        );
                        numbers.push(number);
                    }
                }
            }
            numbers.sort();
            numbers.dedup();
            sum += numbers.iter().sum::<usize>();
        }
    }

    println!("Part 1: {}", sum);
}

fn part2(schematic: Vec<Vec<char>>) {
    let mut sum = 0;
    for (y, _) in schematic.iter().enumerate() {
        for (x, _) in schematic[y].iter().enumerate() {
            if schematic[y][x] != '*' {
                continue;
            }
            let mut numbers: Vec<usize> = Vec::new();
            for a in -1..=1 {
                for b in -1..=1 {
                    let checked_x = x as isize + a;
                    let checked_y = y as isize + b;
                    let c = get_element_or_default(&schematic, checked_x, checked_y);
                    if c.is_digit(10) {
                        let number = get_number_digit_is_part_of(
                            &schematic,
                            checked_x as usize,
                            checked_y as usize,
                        );
                        numbers.push(number);
                    }
                }
            }
            numbers.sort();
            numbers.dedup();
            if numbers.len() == 2 {
                sum += numbers[0] * numbers[1];
            }
        }
    }

    println!("Part 2: {}", sum);
}

fn get_element_or_default(schematic: &Vec<Vec<char>>, x: isize, y: isize) -> char {
    if x < 0 || y < 0 {
        return '.';
    }
    *schematic
        .get(y as usize)
        .unwrap_or(&vec![])
        .get(x as usize)
        .unwrap_or(&'.')
}

fn get_number_digit_is_part_of(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut number = String::from(schematic[y][x]);

    if x > 0 {
        for i in (0..=(x - 1)).rev() {
            let c = get_element_or_default(&schematic, i as isize, y as isize);
            if c.is_digit(10) {
                number = format!("{}{}", c, number);
            } else {
                break;
            }
        }
    }

    for i in (x + 1)..=schematic[y].len() {
        let c = get_element_or_default(&schematic, i as isize, y as isize);
        if c.is_digit(10) {
            number = format!("{}{}", number, c);
        } else {
            break;
        }
    }
    let number: usize = number.parse().unwrap();
    number
}
