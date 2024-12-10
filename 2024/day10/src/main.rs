use std::collections::HashSet;

fn main() {
    let map: Vec<Vec<usize>> = std::fs::read_to_string("input.txt")
        .expect("Failed to read input file")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    part1(map.clone());
    part2(map);
}

fn part1(map: Vec<Vec<usize>>) {
    let trailhead_locations = find_trailhead_locations(&map);
    let scores: usize = trailhead_locations
        .iter()
        .map(|(x, y)| {
            let mut reachable_peaks: HashSet<(usize, usize)> = HashSet::new();
            calculate_score(&map, *x as isize, *y as isize, -1, &mut reachable_peaks);
            reachable_peaks.len()
        })
        .sum();

    println!("Part 1: {}", scores);
}

fn part2(map: Vec<Vec<usize>>) {
    let trailhead_locations = find_trailhead_locations(&map);
    let scores: usize = trailhead_locations
        .iter()
        .map(|(x, y)| calculate_rating(&map, *x as isize, *y as isize, -1))
        .sum();

    println!("Part 2: {}", scores);
}

fn find_trailhead_locations(map: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, value)| **value == 0)
                .map(move |(x, _)| (x, y))
        })
        .collect()
}

fn is_valid_location(map: &Vec<Vec<usize>>, x: isize, y: isize) -> bool {
    x < map[0].len() as isize && y < map.len() as isize && x >= 0 && y >= 0
}

fn calculate_score(
    map: &Vec<Vec<usize>>,
    x: isize,
    y: isize,
    last_level: isize,
    reachable_peaks: &mut HashSet<(usize, usize)>,
) {
    if !is_valid_location(map, x, y) {
        return;
    }
    let current_level = map[y as usize][x as usize];
    if current_level != (last_level + 1) as usize {
        return;
    }
    if current_level == 9 {
        reachable_peaks.insert((x as usize, y as usize));
    }
    calculate_score(map, x + 1, y, current_level as isize, reachable_peaks);
    calculate_score(map, x - 1, y, current_level as isize, reachable_peaks);
    calculate_score(map, x, y + 1, current_level as isize, reachable_peaks);
    calculate_score(map, x, y - 1, current_level as isize, reachable_peaks);
}

fn calculate_rating(map: &Vec<Vec<usize>>, x: isize, y: isize, last_level: isize) -> usize {
    if !is_valid_location(map, x, y) {
        return 0;
    }
    let current_level = map[y as usize][x as usize];
    if current_level != (last_level + 1) as usize {
        return 0;
    }
    if current_level == 9 {
        return 1;
    }
    calculate_rating(map, x + 1, y, current_level as isize)
        + calculate_rating(map, x - 1, y, current_level as isize)
        + calculate_rating(map, x, y + 1, current_level as isize)
        + calculate_rating(map, x, y - 1, current_level as isize)
}
