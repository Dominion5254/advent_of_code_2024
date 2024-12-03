use regex::Regex;
use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read input file");
    let multiplications = parse_dumb_input(input.clone());
    let answer = sum_mults(multiplications, &input);

    println!("Answer: {}", answer);
}

// fn parse_input(input: String) -> Vec<Mult> {
//     let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
//     let matches: Vec<&str> = re.find_iter(&input).map(|m| m.as_str()).collect();
//     matches
//         .iter()
//         .map(|&m| {
//             let first: usize = m.find("(").unwrap();
//             let last: usize = m.find(")").unwrap();
//             println!(
//                 "Substring: {:?}",
//                 m[first + 1..last].split(",").collect::<Vec<&str>>()
//             );
//             let numbers: Vec<u32> = m[first + 1..last]
//                 .split(",")
//                 .collect::<Vec<&str>>()
//                 .iter()
//                 .map(|n| n.parse::<u32>().unwrap())
//                 .collect();
//             Mult {
//                 x: numbers[0],
//                 y: numbers[1],
//             }
//         })
//         .collect()
// }

fn parse_dumb_input(input: String) -> Vec<Mult> {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let matches: Vec<(&str, usize)> = re.find_iter(&input).map(|m| (m.as_str(), m.start())).collect();

    matches
        .iter()
        .map(|&m| {
            let mut first: usize = m.0.find("(").unwrap();
            first += 1;
            let last: usize = m.0.find(")").unwrap();
            println!(
                "Substring: {:?}",
                m.0[first..last].split(",").collect::<Vec<&str>>()
            );
            let numbers: Vec<u32> = m.0[first..last]
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            Mult {
                x: numbers[0],
                y: numbers[1],
                i: m.1,
            }
        })
        .collect()
}

struct Mult {
    x: u32,
    y: u32,
    i: usize,
}

fn sum_mults(mults: Vec<Mult>, input: &str) -> u32 {
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let mut do_matches: VecDeque<usize> = do_re.find_iter(&input).map(|m| m.start()).collect();
    let mut dont_matches: VecDeque<usize> = dont_re.find_iter(&input).map(|m| m.start()).collect();
    println!("Do Matches: {:?}", do_matches);
    println!("Don't Matches: {:?}", dont_matches);

    let mut do_true = true;
    let mut index: usize = 0;
    let mut do_ranges = vec![];
    loop {
        if do_true {
            match dont_matches.pop_front() {
                None => break,
                Some(new_index) => {
                    if new_index.gt(&index) {
                        do_ranges.push(index..new_index);
                        index = new_index;
                        do_true = false;
                    }
                }
            }
        } else {
            match do_matches.pop_front() {
                None => break,
                Some(new_index) => {
                    if new_index.gt(&index) {
                        index = new_index;
                        do_true = true;
                        if dont_matches.is_empty() {
                            do_ranges.push(index..input.len());
                        }
                    }
                }
            }
        }
    }
    println!("Do Ranges: {:?}", &do_ranges);
    let mut sum = 0;
    for mult in mults {
        println!("Mult '{}, {}' at {}", mult.x, mult.y, mult.i);
        'inner: for range in &do_ranges {
            if range.contains(&mult.i) {
                sum += mult.x * mult.y;
                break 'inner;
            }
        }
    }

    sum
}
