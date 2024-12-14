use core::ops::Range;
use std::{fs, ops::Add};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to reaad file");
    let filesystem = parse_input_p2(input);
    // println!("test: {:?}", filesystem);
    // println!("filesystem: {:?}", filesystem);
    let reordered = move_blocks_p2(filesystem);
    let answer = checksum(reordered);

    println!("answer: {}", answer);
}

fn _parse_input(input: String) -> Vec<i64> {
    let mut filesystem: Vec<i64> = vec![];
    let mut filesystem_index: i64 = 0;
    for (index, ch) in input.chars().enumerate() {
        let num = ch.to_digit(10).expect("expected digit");
        if index % 2 == 0 {
            for _ in 0..num {
                filesystem.push(filesystem_index.try_into().unwrap());
            }
            filesystem_index += 1;
        } else {
            for _ in 0..num {
                filesystem.push(-1);
            }
        }
    }

    filesystem
}

fn _move_blocks(filesystem: Vec<i64>) -> Vec<i64> {
    let original_fs = filesystem.clone();
    let reordered = filesystem
        .iter()
        .enumerate()
        .fold(original_fs, |mut acc, (index, &c)| {
            if c == -1 {
                let last_file_idx = acc.iter().rposition(|&e| e != -1).unwrap();
                if last_file_idx < index {
                    return acc;
                }
                acc[index] = acc[last_file_idx];
                acc[last_file_idx] = -1;
            }
            acc
        });

    reordered
}

fn parse_input_p2(input: String) -> Vec<i64> {
    let mut filesystem: Vec<i64> = vec![];
    let mut filesystem_index: i64 = 0;
    let mut free_space_idx: i64 = -1;
    for (index, ch) in input.chars().enumerate() {
        let num = ch.to_digit(10).expect("expected digit");
        if index % 2 == 0 {
            for _ in 0..num {
                filesystem.push(filesystem_index);
            }
            filesystem_index += 1;
        } else {
            for _ in 0..num {
                filesystem.push(free_space_idx);
            }
            free_space_idx -= 1;
        }
    }
    filesystem
}

fn move_blocks_p2(filesystem: Vec<i64>) -> Vec<i64> {
    let mut unique_values = filesystem.clone();
    let mut new_filesystem = filesystem.clone();
    unique_values.dedup();
    let mut file_ranges: Vec<Range<usize>> = vec![];
    let mut free_ranges: Vec<Range<usize>> = vec![];
    for value in unique_values {
        let range = filesystem.iter().position(|&e| e == value).unwrap()
            ..filesystem.iter().rposition(|&e| e == value).unwrap().add(1);
        if value.ge(&0) {
            file_ranges.push(range);
        } else {
            free_ranges.push(range);
        }
    }
    file_ranges.reverse();
    // println!("Filesystem {:?}", filesystem);
    // println!("Free Ranges: {:?}", free_ranges);
    for range in file_ranges {
        let file_length = range.len();
        let mut free_range_iterator = free_ranges.iter().enumerate();
        while let Some((free_range_idx, free_range)) = free_range_iterator.next() {
            // println!("Checking for space for {:?} in {:?}", range, free_range);
            if free_range.start > range.start {
                break;
            }
            match free_range.len() {
                size if size < file_length => continue,
                _ => {
                    let end = free_range.start + file_length;
                    for (range_idx, free_idx) in range.clone().zip(free_range.clone()) {
                        new_filesystem.swap(free_idx, range_idx);
                    }
                    free_ranges[free_range_idx] = end..free_range.end;
                }
            }
            // println!("Free Ranges: {:?}", free_ranges);
            // println!("New Filesystem in loop: {:?}", new_filesystem);
            // new_filesystem.drain(range);
            break;
        }
        // println!("New Filesystem: {:?}", new_filesystem);
    }
    new_filesystem
}

fn checksum(filesystem: Vec<i64>) -> i64 {
    filesystem.iter().enumerate().fold(
        0,
        |acc, (index, &e)| {
            if e.is_negative() {
                acc
            } else {
                acc + index as i64 * e
            }
        },
    )
}
