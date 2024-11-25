use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").expect("Could not open input file");
    let reader = io::BufReader::new(file);

    let mut list1: Vec<usize> = Vec::new();
    let mut list2: Vec<usize> = Vec::new();

    reader.lines().into_iter().for_each(|line| {
        let line = line.expect("Failed to read line");
        let (n1str, n2str) = line.split_once("   ").expect("failed to split line");
        list1.push(n1str.parse().expect("failed to parse number"));
        list2.push(n2str.parse().expect("failed to parse number"));
    });

    part1(list1.clone(), list2.clone());
    part2(list1, list2);
}

fn part1(mut list1: Vec<usize>, mut list2: Vec<usize>) {
    list1.sort();
    list2.sort();

    let sum: usize = list1
        .into_iter()
        .zip(list2.into_iter())
        .map(|(x, y)| x.abs_diff(y))
        .sum();

    println!("Part 1: {}", sum);
}

fn part2(list1: Vec<usize>, list2: Vec<usize>) {
    let mut sum = 0;

    for i in 0..list1.len() {
        let multiplier = list2.iter().filter(|x| **x == list1[i]).count();
        sum += list1[i] * multiplier;
    }

    println!("Part 2: {}", sum);
}
