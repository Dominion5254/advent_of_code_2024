use std::{
    fs,
    ops::{Add, Sub},
};

use diagonal::{diagonal_pos_neg, diagonal_pos_pos};
use regex::Regex;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read file");
    let vecs = parse_input(&input);
    let answer = stupid_elf(vecs);
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

        count += query.find_iter(&text).count();
        count += query_reversed.find_iter(&text).count();
    }

    count
}

fn stupid_elf(vectors: Vec<Vec<Vec<char>>>) -> usize {
    let important_vec = vectors[0].clone();
    let row_count = important_vec.len();
    let col_count = important_vec[0].len();
    let mut xmas_count: usize = 0;

    for row_idx in 1..row_count.sub(1) {
        for col_idx in 1..col_count.sub(1) {
            if important_vec[row_idx][col_idx] == 'A' {
                if important_vec[row_idx.sub(1)][col_idx.sub(1)] == 'M'
                    && important_vec[row_idx.add(1)][col_idx.add(1)] == 'S'
                {
                    if important_vec[row_idx.sub(1)][col_idx.add(1)] == 'M'
                        && important_vec[row_idx.add(1)][col_idx.sub(1)] == 'S'
                    {
                        xmas_count += 1;
                    } else if important_vec[row_idx.sub(1)][col_idx.add(1)] == 'S'
                        && important_vec[row_idx.add(1)][col_idx.sub(1)] == 'M'
                    {
                        xmas_count += 1;
                    }
                } else if important_vec[row_idx.sub(1)][col_idx.sub(1)] == 'S'
                    && important_vec[row_idx.add(1)][col_idx.add(1)] == 'M'
                {
                    if important_vec[row_idx.sub(1)][col_idx.add(1)] == 'M'
                        && important_vec[row_idx.add(1)][col_idx.sub(1)] == 'S'
                    {
                        xmas_count += 1;
                    } else if important_vec[row_idx.sub(1)][col_idx.add(1)] == 'S'
                        && important_vec[row_idx.add(1)][col_idx.sub(1)] == 'M'
                    {
                        xmas_count += 1;
                    }
                }
            }
        }
    }

    xmas_count
}
