fn main() {
    let disk: String = std::fs::read_to_string("input.txt")
        .expect("Failed to read input file")
        .trim()
        .into();
    // char is '1' (file data) or '.' (empty), usize is id
    let expanded_disk: Vec<(char, usize)> = disk
        .chars()
        .enumerate()
        .flat_map(|(id, c)| {
            let parsed_char = c.to_string().parse::<usize>().unwrap();
            if id % 2 == 0 {
                std::iter::repeat(('1', id / 2))
                    .take(parsed_char)
                    .collect::<Vec<(char, usize)>>()
            } else {
                std::iter::repeat(('.', 0))
                    .take(parsed_char)
                    .collect::<Vec<(char, usize)>>()
            }
        })
        .collect();

    part1(expanded_disk.clone());
    part2(disk, expanded_disk);
}

fn part1(mut expanded_disk: Vec<(char, usize)>) {
    let mut ordered_disk: Vec<(char, usize)> = Vec::new();

    // iterate from 0 to len of expanded_disk, which we can calculate
    for i in 0..expanded_disk.iter().filter(|(c, _)| *c == '1').count() {
        let (c, id) = expanded_disk[i];
        if c == '1' {
            ordered_disk.push((c, id));
        } else {
            // found gap, pop '1' from the end of expanded_disk, add to ordered_disk
            while let Some((c, id)) = expanded_disk.pop() {
                if c == '1' {
                    ordered_disk.push((c, id));
                    break;
                }
            }
        }
    }

    let checksum = ordered_disk
        .iter()
        .enumerate()
        .map(|(position, (_, id))| position * id)
        .sum::<usize>();
    println!("Part 1: {}", checksum);
}

fn part2(disk: String, mut expanded_disk: Vec<(char, usize)>) {
    // iterate over ids from highest
    for id in (0..(disk.len() / 2) + 1).rev() {
        // find indices of id
        let id_indices = expanded_disk
            .iter()
            .enumerate()
            .filter(|(_, (c, i))| *c == '1' && *i == id)
            .map(|(i, (_, _))| i)
            .collect::<Vec<usize>>();
        // find dots to fit len of id_indices
        for i in 0..id_indices[0] {
            if expanded_disk[i].0 == '.' {
                // check len of current gap
                let mut gap_len = 1;
                loop {
                    if i + gap_len >= expanded_disk.len() {
                        break;
                    }
                    if expanded_disk[i + gap_len].0 == '.' {
                        gap_len += 1;
                    } else {
                        break;
                    }
                }
                if gap_len >= id_indices.len() {
                    // fitting gap found, moving
                    for i in id_indices.iter() {
                        expanded_disk[*i] = ('.', 0);
                    }
                    for j in 0..id_indices.len() {
                        expanded_disk[i + j] = ('1', id);
                    }
                    break;
                }
            }
        }
    }

    let checksum = expanded_disk
        .iter()
        .enumerate()
        .map(|(position, (_, id))| position * id)
        .sum::<usize>();
    println!("Part 2: {}", checksum);
}
