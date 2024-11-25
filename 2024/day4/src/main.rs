use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").expect("Could not open input file");
    let reader = io::BufReader::new(file);

    let mut puzzle: Vec<Vec<char>> = Vec::new();

    reader.lines().into_iter().for_each(|line| {
        let line = line.expect("Failed to read line");
        let line_chars: Vec<char> = line.chars().collect();
        puzzle.push(line_chars);
    });

    part1(puzzle.clone());
    part2(puzzle);
}

fn part1(puzzle: Vec<Vec<char>>) {
    let mut sum = 0;
    for (y, _) in puzzle.iter().enumerate() {
        for (x, _) in puzzle[y].iter().enumerate() {
            if puzzle[y][x] != 'X' {
                continue;
            }

            // right
            if get_element_or_default(&puzzle, x + 1, y) == 'M'
                && get_element_or_default(&puzzle, x + 2, y) == 'A'
                && get_element_or_default(&puzzle, x + 3, y) == 'S'
            {
                sum += 1;
            }

            // left
            if x >= 3
                && get_element_or_default(&puzzle, x - 1, y) == 'M'
                && get_element_or_default(&puzzle, x - 2, y) == 'A'
                && get_element_or_default(&puzzle, x - 3, y) == 'S'
            {
                sum += 1;
            }

            // down
            if get_element_or_default(&puzzle, x, y + 1) == 'M'
                && get_element_or_default(&puzzle, x, y + 2) == 'A'
                && get_element_or_default(&puzzle, x, y + 3) == 'S'
            {
                sum += 1;
            }

            // up
            if y >= 3
                && get_element_or_default(&puzzle, x, y - 1) == 'M'
                && get_element_or_default(&puzzle, x, y - 2) == 'A'
                && get_element_or_default(&puzzle, x, y - 3) == 'S'
            {
                sum += 1;
            }

            // diagonal down right
            if get_element_or_default(&puzzle, x + 1, y + 1) == 'M'
                && get_element_or_default(&puzzle, x + 2, y + 2) == 'A'
                && get_element_or_default(&puzzle, x + 3, y + 3) == 'S'
            {
                sum += 1;
            }

            // diagonal down left
            if x >= 3
                && get_element_or_default(&puzzle, x - 1, y + 1) == 'M'
                && get_element_or_default(&puzzle, x - 2, y + 2) == 'A'
                && get_element_or_default(&puzzle, x - 3, y + 3) == 'S'
            {
                sum += 1;
            }

            // diagonal up right
            if y >= 3
                && get_element_or_default(&puzzle, x + 1, y - 1) == 'M'
                && get_element_or_default(&puzzle, x + 2, y - 2) == 'A'
                && get_element_or_default(&puzzle, x + 3, y - 3) == 'S'
            {
                sum += 1;
            }

            // diagonal up left
            if x >= 3
                && y >= 3
                && get_element_or_default(&puzzle, x - 1, y - 1) == 'M'
                && get_element_or_default(&puzzle, x - 2, y - 2) == 'A'
                && get_element_or_default(&puzzle, x - 3, y - 3) == 'S'
            {
                sum += 1;
            }
        }
    }
    println!("Part 1: {}", sum);
}

fn part2(puzzle: Vec<Vec<char>>) {
    let mut sum = 0;
    for (y, _) in puzzle.iter().enumerate() {
        for (x, _) in puzzle[y].iter().enumerate() {
            if puzzle[y][x] != 'A' || x < 1 || y < 1 {
                continue;
            }

            let top_left = get_element_or_default(&puzzle, x - 1, y - 1);
            let top_right = get_element_or_default(&puzzle, x + 1, y - 1);
            let bottom_left = get_element_or_default(&puzzle, x - 1, y + 1);
            let bottom_right = get_element_or_default(&puzzle, x + 1, y + 1);

            if ((top_left == 'M' && bottom_right == 'S')
                || (top_left == 'S' && bottom_right == 'M'))
                && ((top_right == 'M' && bottom_left == 'S')
                    || (top_right == 'S' && bottom_left == 'M'))
            {
                sum += 1;
            }
        }
    }
    println!("Part 2: {}", sum);
}

fn get_element_or_default(puzzle: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    *puzzle.get(y).unwrap_or(&vec![]).get(x).unwrap_or(&'.')
}
