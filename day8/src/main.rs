use std::fs::File;
use std::io::BufRead;

fn main() -> color_eyre::Result<()> {
    let file = File::open("src/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut forest: Vec<Vec<u32>> = Vec::new();

    for line in reader.lines() {
        let data = line.unwrap();

        let mut row: Vec<u32> = Vec::new();
        for c in data.chars() {
            let tree_height = c.to_digit(10).unwrap();
            row.push(tree_height);
        }
        forest.push(row);
    }

    // Part 1
    let mut visible_trees = 0;

    for row_index in 0..forest.len() {
        for column_index in 0..forest[row_index].len() {
            if tree_is_visible(row_index, column_index, &forest) {
                visible_trees += 1;
            }
        }
    }

    println!("part 1: {}", visible_trees);

    // Part 2
    let mut scenic_scores: Vec<u32> = Vec::new();

    for row_index in 0..forest.len() {
        for column_index in 0..forest[row_index].len() {
            scenic_scores.push(calculate_scenic_score(row_index, column_index, &forest));
        }
    }

    println!("part 2: {:?}", scenic_scores.iter().max().unwrap());

    Ok(())
}

fn calculate_scenic_score(row_index: usize, column_index: usize, forest: &Vec<Vec<u32>>) -> u32 {
    let mut scenic_score = 0;

    if !tree_is_on_edge(row_index, column_index, forest) {
        let tree_size = forest[row_index][column_index];

        let mut score_left = 0;
        let mut score_right = 0;
        let mut score_up = 0;
        let mut score_down = 0;

        for index in (0..column_index).rev() {
            score_left += 1;
            if forest[row_index][index] >= tree_size {
                break;
            }
        }

        for index in column_index + 1..forest[0].len() {
            score_right += 1;
            if forest[row_index][index] >= tree_size {
                break;
            }
        }

        for index in (0..row_index).rev() {
            score_up += 1;
            if forest[index][column_index] >= tree_size {
                break;
            }
        }

        for row in forest.iter().take(forest[0].len()).skip(row_index + 1) {
            score_down += 1;
            if row[column_index] >= tree_size {
                break;
            }
        }

        scenic_score = score_left * score_right * score_down * score_up;
    }

    scenic_score
}

fn tree_is_visible(row_index: usize, column_index: usize, forest: &Vec<Vec<u32>>) -> bool {
    tree_is_on_edge(row_index, column_index, forest)
        || tree_is_tallest(row_index, column_index, forest)
}

fn tree_is_on_edge(row_index: usize, column_index: usize, forest: &Vec<Vec<u32>>) -> bool {
    let mut is_visible = false;

    if row_index == 0
        || row_index == forest.len() - 1
        || column_index == 0
        || column_index == forest[row_index].len() - 1
    {
        is_visible = true;
    }

    is_visible
}

fn tree_is_tallest(row_index: usize, column_index: usize, forest: &[Vec<u32>]) -> bool {
    tree_is_tallest_up(row_index, column_index, forest)
        || tree_is_tallest_down(row_index, column_index, forest)
        || tree_is_tallest_left(row_index, column_index, forest)
        || tree_is_tallest_right(row_index, column_index, forest)
}

fn tree_is_tallest_up(row_index: usize, column_index: usize, forest: &[Vec<u32>]) -> bool {
    let mut is_visible = true;

    let tree_height = forest[row_index][column_index];

    // start at the upper bound
    let mut index = row_index - 1;

    loop {
        let check_height = forest[index][column_index];

        if check_height >= tree_height {
            is_visible = false;
            break;
        }

        if index == 0 {
            break;
        }
        index -= 1;
    }

    is_visible
}

fn tree_is_tallest_down(row_index: usize, column_index: usize, forest: &[Vec<u32>]) -> bool {
    let mut is_visible = true;

    let tree_height = forest[row_index][column_index];

    for row in forest
        .iter()
        .take(forest[row_index].len())
        .skip(row_index + 1)
    {
        let check_height = row[column_index];

        if check_height >= tree_height {
            is_visible = false;
            break;
        }
    }

    is_visible
}

fn tree_is_tallest_left(row_index: usize, column_index: usize, forest: &[Vec<u32>]) -> bool {
    let mut is_visible = true;

    let tree_height = forest[row_index][column_index];

    // start at the upper bound
    let mut index = column_index - 1;

    loop {
        let check_height = forest[row_index][index];

        if check_height >= tree_height {
            is_visible = false;
            break;
        }

        if index == 0 {
            break;
        }
        index -= 1;
    }

    is_visible
}

fn tree_is_tallest_right(row_index: usize, column_index: usize, forest: &[Vec<u32>]) -> bool {
    let mut is_visible = true;

    let tree_height = forest[row_index][column_index];

    for index in column_index + 1..forest[row_index].len() {
        let check_height = forest[row_index][index];

        if check_height >= tree_height {
            is_visible = false;
            break;
        }
    }

    is_visible
}
