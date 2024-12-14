use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to reaad file");
    let filesystem = parse_input(input);
    // println!("test: {:?}", filesystem);
    // println!("filesystem: {:?}", filesystem);
    let reordered = move_blocks(filesystem);
    let answer = checksum(reordered);

    println!("answer: {}", answer);
}

fn parse_input(input: String) -> Vec<i64> {
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
 
fn move_blocks(filesystem: Vec<i64>) -> Vec<i64> {
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

fn checksum(filesystem: Vec<i64>) -> i64 {
    filesystem.iter().enumerate().fold(0, |acc, (index, &e)| {
        if e == -1 {
            acc
        } else {
            acc + index as i64 * e
        }
    })
}
