use std::fs;

use diagonal::{diagonal_pos_neg, diagonal_pos_pos};
use regex::{Match, Regex};

fn main() {
    let input = fs::read_to_string("./example.txt").expect("Failed to read file");
    let vecs = parse_input(&input);
    let answer = fucking_diagonals(vecs);
    println!("Result: {:?}", answer);
}

fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    let left_right = input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let num_rows = left_right.len();
    let num_cols = left_right[0].len();

    let mut up_down: Vec<Vec<char>> = vec![vec![]; num_cols];

    for row_index in 0..num_rows {
        for col_index in 0..num_cols {
            up_down[col_index].push(left_right[row_index][col_index]);
        }
    }

    let diag_one = diagonal_pos_pos(&left_right)
        .iter()
        .map(|r| r.iter().map(|&c| c.to_owned()).collect())
        .collect();
    let diag_two = diagonal_pos_neg(&left_right)
        .iter()
        .map(|r| r.iter().map(|&c| c.to_owned()).collect())
        .collect();

    vec![left_right, up_down, diag_one, diag_two]
}

fn count_xmas(vectors: Vec<Vec<Vec<char>>>) -> usize {
    let query = Regex::new(r"XMAS").unwrap();
    let query_reversed = Regex::new(r"SAMX").unwrap();
    let mut count: usize = 0;

    for vector in vectors {
        let text = vector
            .iter()
            .map(|inner| inner.into_iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        count +=  query.find_iter(&text).count();
        count +=  query_reversed.find_iter(&text).count();
    }

    count
}

fn fucking_diagonals(vectors: Vec<Vec<Vec<char>>>) -> usize {
    let important_diag = &vectors[2];
    let query = Regex::new(r"MAS").unwrap();
    let query_reversed = Regex::new(r"SAM").unwrap();
    let mut count: usize = 0;
    let text = important_diag
        .iter()
        .map(|inner| inner.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    println!("Text:\n{}", text);
    // let initial_matches = query_reversed.find_iter(&text).collect::<Vec<Match>>();
    // for m in initial_matches {
    //     println!("Matched {} with 'A' at index {}", m.as_str(), m.start() + 1);
    // }

    for group in important_diag.windows(3) {
        
    }

    10
}