fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    let seeds: Vec<usize> = sections[0].split(": ").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let maps: Vec<Vec<(usize, usize, usize)>> =
        sections[1..].iter().map(|x| parse_map(x)).collect();

    println!("{:?}", maps);

    //part1(map.clone());
    //part2(map);
}

//fn part1(map: Vec<Vec<usize>>) {
//}

//fn part2(map: Vec<Vec<usize>>) {
//}

fn parse_map(map: &str) -> Vec<(usize, usize, usize)> {
    map.lines().collect::<Vec<&str>>()[1..]
        .iter()
        .map(|x| {
            let mut numbers = x
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (
                numbers.pop().unwrap(),
                numbers.pop().unwrap(),
                numbers.pop().unwrap(),
            )
        })
        .collect()
}
