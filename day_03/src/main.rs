use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read input file");
    let multiplications = parse_input(input);
    let answer = sum_mults(multiplications);

    println!("Answer: {}", answer);
}

fn parse_input(input: String) -> Vec<Mult> {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let matches: Vec<&str> = re.find_iter(&input).map(|m| m.as_str()).collect();
    matches
        .iter()
        .map(|&m| {
            let first: usize = m.find("(").unwrap();
            let last: usize = m.find(")").unwrap();
            println!(
                "Substring: {:?}",
                m[first + 1..last].split(",").collect::<Vec<&str>>()
            );
            let numbers: Vec<u32> = m[first + 1..last]
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            Mult {
                x: numbers[0],
                y: numbers[1],
            }
        })
        .collect()
}

struct Mult {
    x: u32,
    y: u32,
}

fn sum_mults(mults: Vec<Mult>) -> u32 {
    let mut sum = 0;
    for mult in mults {
        sum += mult.x * mult.y
    }

    sum
}
