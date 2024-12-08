use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").expect("Could not open input file");
    let reader = io::BufReader::new(file);

    let map: Vec<Vec<char>> = reader
        .lines()
        .into_iter()
        .map(|line| line.expect("Failed to read line").chars().collect())
        .collect();
    let mut antenna_locations: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != '.' {
                antenna_locations
                    .entry(*c)
                    .or_insert(Vec::new())
                    .push((x as isize, y as isize));
            }
        }
    }

    part1(map.clone(), antenna_locations.clone());
    part2(map, antenna_locations);
}

fn part1(map: Vec<Vec<char>>, antenna_locations: HashMap<char, Vec<(isize, isize)>>) {
    let mut antinode_locations: HashSet<(isize, isize)> = HashSet::new();
    antenna_locations.iter().for_each(|(_, locations)| {
        for (id, (x1, y1)) in locations.iter().enumerate() {
            for (x2, y2) in locations.iter().skip(id + 1) {
                let vx = x2 - x1;
                let vy = y2 - y1;
                let p1 = (x1 - vx, y1 - vy);
                if is_valid_point(&map, p1.0, p1.1) {
                    antinode_locations.insert(p1);
                }
                let p2 = (x2 + vx, y2 + vy);
                if is_valid_point(&map, p2.0, p2.1) {
                    antinode_locations.insert(p2);
                }
            }
        }
    });
    println!("Part 1: {}", antinode_locations.len());
}

fn part2(mut map: Vec<Vec<char>>, antenna_locations: HashMap<char, Vec<(isize, isize)>>) {
    antenna_locations.iter().for_each(|(_, locations)| {
        for (id, (x1, y1)) in locations.iter().enumerate() {
            for (x2, y2) in locations.iter().skip(id + 1) {
                let vx = x2 - x1;
                let vy = y2 - y1;

                let mut x1c: isize = *x1;
                let mut y1c: isize = *y1;
                while is_valid_point(&map, x1c, y1c) {
                    map[y1c as usize][x1c as usize] = 'X';
                    x1c -= vx;
                    y1c -= vy;
                }

                let mut x2c: isize = *x2;
                let mut y2c: isize = *y2;
                while is_valid_point(&map, x2c, y2c) {
                    map[y2c as usize][x2c as usize] = 'X';
                    x2c += vx;
                    y2c += vy;
                }
            }
        }
    });

    let mut count = 0;
    for line in map.iter() {
        for c in line.iter() {
            if *c != '.' {
                count += 1;
            }
        }
    }
    println!("Part 2: {}", count);
}

fn is_valid_point(map: &Vec<Vec<char>>, x: isize, y: isize) -> bool {
    x >= 0 && y >= 0 && y < map.len() as isize && x < map[0].len() as isize
}
