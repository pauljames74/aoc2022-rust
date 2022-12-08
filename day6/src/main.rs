use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let data = line.unwrap();

        println!("part 1: {}", identify_start_of_packet(&data, 4));
        println!("part 2: {}", identify_start_of_packet(&data, 14));
    }

    Ok(())
}

fn identify_start_of_packet(data: &str, sequence_length: usize) -> usize {
    let mut packet_start = 0;
    let mut sequence: Vec<char> = Vec::new();

    for (i, c) in data.char_indices() {
        sequence.push(c);
        if sequence.len() > sequence_length {
            sequence.drain(0..1);
        }

        if sequence.len() == sequence_length
            && sequence
                .clone()
                .into_iter()
                .collect::<HashSet<char>>()
                .len()
                == sequence_length
        {
            packet_start = i + 1;
            break;
        }
    }

    packet_start
}
