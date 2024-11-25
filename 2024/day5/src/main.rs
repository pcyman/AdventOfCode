use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").expect("Could not open input file");
    let reader = io::BufReader::new(file);

    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();

    let mut rules_complete = false;
    reader.lines().into_iter().for_each(|line| {
        let line = line.expect("Failed to read line");
        if line == "" {
            rules_complete = true;
            return;
        }
        if !rules_complete {
            let parts: Vec<&str> = line.split("|").collect();
            match rules.get_mut(&parts[0].parse::<usize>().unwrap()) {
                Some(v) => {
                    v.push(parts[1].parse::<usize>().unwrap());
                }
                None => {
                    rules.insert(
                        parts[0].parse::<usize>().unwrap(),
                        vec![parts[1].parse::<usize>().unwrap()],
                    );
                }
            }
        } else {
            updates.push(
                line.split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect(),
            );
        }
    });

    part1(rules.clone(), updates.clone());
    part2(rules, updates);
}

fn part1(rules: HashMap<usize, Vec<usize>>, updates: Vec<Vec<usize>>) {
    let sum: usize = updates
        .iter()
        .filter(|update| is_update_correct(&rules, update))
        .map(|update| update[update.len() / 2])
        .sum();
    println!("Part 1: {}", sum);
}

fn part2(rules: HashMap<usize, Vec<usize>>, updates: Vec<Vec<usize>>) {
    let sum: usize = updates
        .iter()
        .filter(|update| !is_update_correct(&rules, update))
        .map(|update| sort_update(&rules, update))
        .map(|update| update[update.len() / 2])
        .sum();
    println!("Part 2: {}", sum);
}

fn is_update_correct(rules: &HashMap<usize, Vec<usize>>, update: &Vec<usize>) -> bool {
    update == &sort_update(rules, update)
}

fn sort_update(rules: &HashMap<usize, Vec<usize>>, update: &Vec<usize>) -> Vec<usize> {
    let mut sorted = update.clone();
    sorted.sort_by(|a, b| match rules.get(a) {
        Some(rules) => {
            if rules.contains(b) {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Equal;
            }
        }
        None => std::cmp::Ordering::Equal,
    });
    sorted
}
