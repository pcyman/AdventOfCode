use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").expect("Could not open input file");
    let reader = io::BufReader::new(file);

    let mut reports: Vec<Vec<usize>> = Vec::new();

    reader.lines().into_iter().for_each(|line| {
        let line = line.expect("Failed to read line");
        let levels: Vec<usize> = line
            .split_whitespace()
            .map(|x| x.parse().expect("failed to parse number"))
            .collect();
        reports.push(levels);
    });

    part1(reports.clone());
    part2(reports);
}

fn part1(reports: Vec<Vec<usize>>) {
    let result = reports
        .iter()
        .filter(|report| is_report_valid(report))
        .count();
    println!("Part 1: {}", result);
}

fn part2(reports: Vec<Vec<usize>>) {
    let result = reports
        .iter()
        .filter(|report| {
            if is_report_valid(report) {
                true
            } else {
                report.iter().enumerate().any(|(id, _)| {
                    let mut report_copy: Vec<usize> = (*report).clone();
                    report_copy.remove(id);
                    is_report_valid(&report_copy)
                })
            }
        })
        .count();
    println!("Part 2: {}", result);
}

fn is_report_valid(report: &Vec<usize>) -> bool {
    let levels_increasing = report
        .windows(2)
        .all(|w| w[0] < w[1] && w[1].abs_diff(w[0]) <= 3);
    let levels_decreasing = report
        .windows(2)
        .all(|w| w[0] > w[1] && w[0].abs_diff(w[1]) <= 3);
    levels_increasing || levels_decreasing
}
