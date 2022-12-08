use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut raw_cargo: Vec<String> = Vec::new();
    let mut max_positions: usize = 0;
    let mut moves: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let line_type = get_line_type(&line);

        match line_type {
            "cargo" => {
                raw_cargo.push(line);
            }
            "positions" => {
                max_positions =
                    usize::from_str_radix(line.trim().split(' ').last().unwrap(), 16).unwrap();
            }
            "moves" => {
                moves.push(parse_moves(&line));
            }
            _ => {}
        }
    }

    let mut cargo_crates_part_1: Vec<Vec<String>> = populate_cargo(&raw_cargo, max_positions);
    let mut cargo_crates_part_2: Vec<Vec<String>> = cargo_crates_part_1.clone();

    process_moves_cratemover_9000(&moves, &mut cargo_crates_part_1);
    process_moves_cratemover_9001(&moves, &mut cargo_crates_part_2);

    println!("part 1: {}", get_top_of_each_stack(cargo_crates_part_1));
    println!("part 2: {}", get_top_of_each_stack(cargo_crates_part_2));

    Ok(())
}

fn get_top_of_each_stack(cargo_crates: Vec<Vec<String>>) -> String {
    let mut top_crates = String::new();

    for mut stack in cargo_crates {
        top_crates.push_str(&stack.pop().unwrap());
    }

    top_crates
}

fn process_moves_cratemover_9001(moves: &Vec<Vec<i32>>, cargo_crates: &mut [Vec<String>]) {
    for individual_move in moves {
        let quantity: usize = individual_move[0] as usize;
        let from: usize = individual_move[1] as usize - 1;
        let to: usize = individual_move[2] as usize - 1;

        let in_transit: Vec<String> = cargo_crates[from].drain(cargo_crates[from].len() - quantity..).collect();
        
        cargo_crates[to].extend(in_transit);
    }
}

fn process_moves_cratemover_9000(moves: &Vec<Vec<i32>>, cargo_crates: &mut [Vec<String>]) {
    for individual_move in moves {
        for _i in 0..individual_move[0] {
            let from: usize = individual_move[1] as usize - 1;
            let to: usize = individual_move[2] as usize - 1;

            let in_transit = cargo_crates[from].pop().unwrap();
            cargo_crates[to].push(in_transit);
        }

    }
}

fn populate_cargo(input: &[String], needed_vectors: usize) -> Vec<Vec<String>> {
    // Create the needed stacks
    let mut cargo_stacks: Vec<Vec<String>> = vec![vec![]; needed_vectors];

    // Reverse the stacks, so we can process them in the right order
    let mut reversed_input = input.to_vec();
    reversed_input.reverse();

    for input_line in reversed_input {
        // Add a trailing space so we can assure each cargo is the right size
        let input_line = format!("{} ", input_line);

        // Read it four characters at a time
        for (stack_index, chunk) in input_line.as_bytes().chunks(4).enumerate() {
            let crate_contents = chunk[1] as char;

            if crate_contents != ' ' {
                cargo_stacks[stack_index].push(crate_contents.to_string());
            }
        }
    }

    cargo_stacks
}

fn parse_moves(input: &str) -> Vec<i32> {
    let elements: Vec<&str> = input.split_whitespace().collect();

    let num1: i32 = elements[1].parse().unwrap();
    let num2: i32 = elements[3].parse().unwrap();
    let num3: i32 = elements[5].parse().unwrap();

    vec![num1, num2, num3]
}

fn get_line_type(line: &str) -> &str {
    match line.trim_start().chars().next() {
        Some('[') => "cargo",
        Some('1') => "positions",
        Some('m') => "moves",
        _ => "ignore",
    }
}
