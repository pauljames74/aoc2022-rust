use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;

#[derive(Debug)]
struct Move {
    direction: char,
    steps: i32,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();

    solve_part_1("src/input.txt", 2);
    solve_part_2("src/input.txt", 10);

    Ok(())
}

fn solve_part_1(filename: &str, num_positions: u16) {
    let rope: Vec<Position> = generate_rope(num_positions);

    let positions_visited: HashSet<Position> = solve(filename, rope);

    println!("part 1: positions_visited: {}", positions_visited.len());
}

fn solve_part_2(filename: &str, num_positions: u16) {
    let rope: Vec<Position> = generate_rope(num_positions);

    let positions_visited: HashSet<Position> = solve(filename, rope);

    println!("part 2: positions_visited: {}", positions_visited.len());
}

fn generate_rope(num_positions: u16) -> Vec<Position> {
    let mut rope: Vec<Position> = Vec::new();

    for _i in 0..num_positions {
        rope.push(Position { x: 0, y: 0 });
    }

    rope
}

fn solve(filename: &str, mut rope: Vec<Position>) -> HashSet<Position> {
    let mut positions_visited: HashSet<Position> = HashSet::new();

    let file = File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let next_move: Move = get_next_move(line);

        perform_steps(next_move, &mut rope, &mut positions_visited);
    }

    positions_visited
}

fn get_next_move(line: Result<String, std::io::Error>) -> Move {
    let line = line.unwrap();
    let (direction, steps) = line.split_once(' ').unwrap();

    Move {
        direction: direction.chars().next().unwrap(),
        steps: steps.parse().unwrap(),
    }
}

fn perform_steps(
    next_move: Move,
    rope: &mut Vec<Position>,
    positions_visited: &mut HashSet<Position>,
) {
    for _step in 0..next_move.steps {
        match next_move.direction {
            'U' => {
                rope[0].y += 1;
            }
            'D' => {
                rope[0].y -= 1;
            }
            'R' => {
                rope[0].x += 1;
            }
            'L' => {
                rope[0].x -= 1;
            }
            _ => {}
        }

        for i in 1..rope.len() {
            let x_diff = rope[i - 1].x - rope[i].x;
            let y_diff = rope[i - 1].y - rope[i].y;

            let (dx, dy) = match (x_diff, y_diff) {
                // overlapping
                (0, 0) => (0, 0),
                // touching up/left/down/right
                (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
                // touching diagonally
                (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
                // need to move up/left/down/right
                (0, 2) => (0, 1),
                (0, -2) => (0, -1),
                (2, 0) => (1, 0),
                (-2, 0) => (-1, 0),
                // need to move to the right diagonally
                (2, 1) => (1, 1),
                (2, -1) => (1, -1),
                // need to move to the left diagonally
                (-2, 1) => (-1, 1),
                (-2, -1) => (-1, -1),
                // need to move up/down diagonally
                (1, 2) => (1, 1),
                (-1, 2) => (-1, 1),
                (1, -2) => (1, -1),
                (-1, -2) => (-1, -1),
                // 🆕 need to move diagonally
                (-2, -2) => (-1, -1),
                (-2, 2) => (-1, 1),
                (2, -2) => (1, -1),
                (2, 2) => (1, 1),
                _ => panic!("unhandled case: x_diff:y_diff = {x_diff:?}:{y_diff:?}"),
            };

            rope[i].x += dx;
            rope[i].y += dy;

            if i == rope.len() - 1 {
                positions_visited.insert(rope[i].clone());
            }
        }
    }
}
