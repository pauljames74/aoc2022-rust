use std::fs::File;
use std::io::BufRead;

#[derive(Debug)]
struct Signal {
    increment: i32,
    cycles: i32,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();

    let file = File::open("src/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut cycles = 0;
    let mut register: i32 = 1;
    let mut signal_strengths: i32 = 0;

    let mut crt: Vec<Vec<char>> = Vec::new();
    let mut row: Vec<char> = Vec::new();

    for line in reader.lines() {
        let signal: Signal = derive_signal(line);

        for i in 0..signal.cycles {
            cycles += 1;

            if cycles == 20 || (cycles - 20) % 40 == 0 {
                signal_strengths += register * cycles;
            }

            if row.len() == 40 {
                crt.push(row.clone());
                row.clear();
            }

            let crt_lower_bound: i32 = register - 1;
            let crt_upper_bound: i32 = register + 1;
            let crt_len: i32 = row.len() as i32;

            if crt_len >= crt_lower_bound && crt_len <= crt_upper_bound {
                row.push('#');
            } else {
                row.push('.');
            }

            if i == signal.cycles - 1 {
                register += signal.increment;
            }
        }
    }
    crt.push(row.clone());

    println!("part 1: {}", signal_strengths);

    println!("\n\npart 2:");
    for crt_row in crt {
        for row in crt_row.chunks(40).map(String::from_iter) {
            println!("row: {}", row);
        }
    }

    Ok(())
}

fn derive_signal(line: Result<String, std::io::Error>) -> Signal {
    let line = line.unwrap();

    let elements: Vec<&str> = line.split_whitespace().collect();

    match elements[0] {
        "noop" => Signal {
            increment: 0,
            cycles: 1,
        },
        "addx" => Signal {
            increment: elements[1].parse::<i32>().unwrap(),
            cycles: 2,
        },
        _ => panic!("unhandled instruction: instruction = {elements:?}"),
    }
}
