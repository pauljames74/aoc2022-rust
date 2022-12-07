use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut elves = Vec::new();

    let mut current_elf = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
            continue;
        }

        let value = line
            .parse::<u64>()
            .expect("Failed to parse line as an integer");
        current_elf += value;
    }

    // Sort descending
    elves.sort_by(|a, b| b.cmp(a));

    println!(
        "The elf with the most calories is carrying {} calories",
        &elves[0]
    );

    println!(
        "The top 3 elves are carring {} calories",
        elves[0] + elves[1] + elves[2]
    );

    Ok(())
}
