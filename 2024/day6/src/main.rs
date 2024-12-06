use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").expect("Could not open input file");
    let reader = io::BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();

    reader.lines().into_iter().for_each(|line| {
        let line = line.expect("Failed to read line");
        let line_chars: Vec<char> = line.chars().collect();
        map.push(line_chars);
    });

    part1(map.clone());
    part2(map);
}

fn part1(mut map: Vec<Vec<char>>) {
    let mut player_location_x: usize = 0;
    let mut player_location_y: usize = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '^' {
                player_location_x = x;
                player_location_y = y;
            }
        }
    }
    let mut visited_locations: HashSet<(usize, usize)> = HashSet::new();
    loop {
        visited_locations.insert((player_location_x, player_location_y));
        let next_tile = get_next_tile(&map, player_location_x, player_location_y);
        if next_tile == '!' {
            break;
        }
        if next_tile == '.' {
            (player_location_x, player_location_y) =
                move_player(&mut map, player_location_x, player_location_y);
        } else {
            rotate_player(&mut map, player_location_x, player_location_y);
        }
    }
    println!("Part 1: {}", visited_locations.len());
}

fn part2(map: Vec<Vec<char>>) {
    let mut player_location_x: usize = 0;
    let mut player_location_y: usize = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '^' {
                player_location_x = x;
                player_location_y = y;
            }
        }
    }
    let mut sum: usize = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '.' {
                let mut counter = 0;
                let mut map_copy = map.clone();
                let mut player_location_x_copy = player_location_x;
                let mut player_location_y_copy = player_location_y;
                map_copy[y][x] = '#';
                loop {
                    if counter == 10000 {
                        sum += 1;
                        break;
                    }
                    let next_tile =
                        get_next_tile(&map_copy, player_location_x_copy, player_location_y_copy);
                    if next_tile == '!' {
                        break;
                    }
                    if next_tile == '.' {
                        (player_location_x_copy, player_location_y_copy) = move_player(
                            &mut map_copy,
                            player_location_x_copy,
                            player_location_y_copy,
                        );
                    } else {
                        rotate_player(
                            &mut map_copy,
                            player_location_x_copy,
                            player_location_y_copy,
                        );
                    }
                    counter += 1;
                }
            }
        }
    }
    println!("Part 2: {}", sum);
}

fn get_element_or_default(map: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    *map.get(y).unwrap_or(&vec![]).get(x).unwrap_or(&'!')
}

fn get_next_tile(map: &Vec<Vec<char>>, player_x: usize, player_y: usize) -> char {
    let direction = map[player_y][player_x];
    match direction {
        '^' => {
            if player_y == 0 {
                return '!';
            }
            get_element_or_default(map, player_x, player_y - 1)
        }
        'v' => get_element_or_default(map, player_x, player_y + 1),
        '<' => {
            if player_x == 0 {
                return '!';
            }
            get_element_or_default(map, player_x - 1, player_y)
        }
        '>' => get_element_or_default(map, player_x + 1, player_y),
        _ => panic!("Unknown direction: {}", direction),
    }
}

fn rotate_player(map: &mut Vec<Vec<char>>, player_x: usize, player_y: usize) {
    let direction = map[player_y][player_x];
    match direction {
        '^' => {
            map[player_y][player_x] = '>';
        }
        'v' => {
            map[player_y][player_x] = '<';
        }
        '<' => {
            map[player_y][player_x] = '^';
        }
        '>' => {
            map[player_y][player_x] = 'v';
        }
        _ => panic!("Unknown direction: {}", direction),
    }
}

fn move_player(map: &mut Vec<Vec<char>>, player_x: usize, player_y: usize) -> (usize, usize) {
    let direction = map[player_y][player_x];
    match direction {
        '^' => {
            map[player_y][player_x] = '.';
            map[player_y - 1][player_x] = '^';
            (player_x, player_y - 1)
        }
        'v' => {
            map[player_y][player_x] = '.';
            map[player_y + 1][player_x] = 'v';
            (player_x, player_y + 1)
        }
        '<' => {
            map[player_y][player_x] = '.';
            map[player_y][player_x - 1] = '<';
            (player_x - 1, player_y)
        }
        '>' => {
            map[player_y][player_x] = '.';
            map[player_y][player_x + 1] = '>';
            (player_x + 1, player_y)
        }
        _ => panic!("Unknown direction: {}", direction),
    }
}
