use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("src/strategy.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut total_score = 0;
    let mut revised_strategy_score = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let mut split_parts = line.split(" ");
        let opponent_code = split_parts.next().unwrap();
        let player_code = split_parts.next().unwrap();

        let opponent = match opponent_code {
            "A" => "rock",
            "B" => "paper",
            "C" => "scissors",
            _ => "unknown"
        };

        let player = match player_code {
            "X" => "rock",
            "Y" => "paper",
            "Z" => "scissors",
            _ => "unknown"
        };

        total_score += calculate_score(opponent, player);

        // Part 2
        let result_wanted = match player_code {
            "X" => "lose",
            "Y" => "draw",
            "Z" => "win",
            _ => "unknown"
        };

        let shape_wanted = match (opponent, result_wanted) {
            ("rock", "lose") => "scissors",
            ("rock", "draw") => "rock",
            ("rock", "win") => "paper",
            ("paper", "lose") => "rock",
            ("paper", "draw") => "paper",
            ("paper", "win") => "scissors",
            ("scissors", "lose") => "paper",
            ("scissors", "draw") => "scissors",
            ("scissors", "win") => "rock",
            _ => "unknown"
        };

        revised_strategy_score += calculate_score(opponent, shape_wanted);
    }

    println!("part 1 score: {}", total_score);
    println!("part 2 score: {}", revised_strategy_score);

    Ok(())
}

fn calculate_score(opponent: &str, player: &str) -> i32 {
    // Compare the strings
    let outcome_score = match (opponent, player) {
        ("rock", "rock") => 3,
        ("rock", "paper") => 6,
        ("rock", "scissors") => 0,
        ("paper", "rock") => 0,
        ("paper", "paper") => 3,
        ("paper", "scissors") => 6,
        ("scissors", "rock") => 6,
        ("scissors", "paper") => 0,
        ("scissors", "scissors") => 3,
        _ => 0
    };

    let shape_score = match player {
        "rock" => 1,
        "paper" => 2,
        "scissors" => 3,
        _ => 0
    };

    let round_score = outcome_score + shape_score;

    return round_score;
}