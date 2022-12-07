use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut part_1_total = 0;
    let mut part_2_total = 0;

    let mut rucksack_group: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line?;

        // Part 1
        let (first_half, second_half) = line.split_at(line.len() / 2);

        let first_items: HashSet<char> = HashSet::from_iter(first_half.chars());
        let second_items: HashSet<char> = HashSet::from_iter(second_half.chars());

        let common_items: &char = first_items
            .intersection(&second_items)
            .collect::<Vec<&char>>()[0];

        part_1_total += priority(*common_items);

        // Part 2
        rucksack_group.push(line);

        if rucksack_group.len() == 3 {
            let first_group: HashSet<char> = HashSet::from_iter(rucksack_group[0].chars());
            let second_group: HashSet<char> = HashSet::from_iter(rucksack_group[1].chars());
            let third_group: HashSet<char> = HashSet::from_iter(rucksack_group[2].chars());

            let common_group_items: char = first_group
                .intersection(&second_group)
                .copied()
                .collect::<HashSet<char>>()
                .intersection(&third_group)
                .copied()
                .collect::<Vec<char>>()[0];

            part_2_total += priority(common_group_items);

            rucksack_group.clear();
        }
    }

    println!("part_1_total: {}", part_1_total);
    println!("part_2_total: {}", part_2_total);

    Ok(())
}

fn priority(c: char) -> i32 {
    let ascii_value = c as i32;

    if c.is_lowercase() {
        ascii_value - ('a' as i32) + 1
    } else {
        ascii_value - ('A' as i32) + 27
    }
}
