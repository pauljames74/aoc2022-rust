use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut fully_contained = 0;
    let mut overlap = 0;

    for line in reader.lines() {
        let line = line?;

        let mut split_parts = line.split(',');

        let range1: Range<i32> = get_range(split_parts.next().unwrap());
        let range2: Range<i32> = get_range(split_parts.next().unwrap());

        // Part 1
        if is_range_fully_contained(&range1, &range2) {
            fully_contained += 1;
        }

        // Part 2
        if has_overlap(&range1, &range2) {
            overlap += 1;
        }
    }

    println!("part 1: {}", fully_contained);
    println!("part 2: {}", overlap);

    Ok(())
}

fn get_range(range_string: &str) -> Range<i32> {
    let mut range_parts = range_string.split('-');

    let part_1 = range_parts.next().unwrap().parse::<i32>().unwrap();
    let part_2 = range_parts.next().unwrap().parse::<i32>().unwrap();

    part_1..part_2
}

fn is_range_fully_contained(range_a: &Range<i32>, range_b: &Range<i32>) -> bool {
    range_a.start >= range_b.start && range_a.end <= range_b.end
        || range_b.start >= range_a.start && range_b.end <= range_a.end
}

fn has_overlap(range_a: &Range<i32>, range_b: &Range<i32>) -> bool {
    range_a.start >= range_b.start && range_a.start <= range_b.end
        || range_b.start >= range_a.start && range_b.start <= range_a.end
}
