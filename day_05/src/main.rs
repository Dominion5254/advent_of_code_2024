use std::{fs, ops::{Div, Sub}};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read file");
    let (rules, updates) = parse_input(input);
    let answer = sum_valid_updates(rules, updates);

    println!("Answer: {}", answer);
}

fn parse_input(input: String) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let (rules_input, updates_input) = input.split_once("\n\n").unwrap();
    let rules: Vec<(u32, u32)> = rules_input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&rule| {
            let rule = rule.split_once("|").unwrap();
            (
                rule.0.parse::<u32>().unwrap(),
                rule.1.parse::<u32>().unwrap(),
            )
        })
        .collect();

    let updates: Vec<Vec<u32>> = updates_input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&line| {
            line.split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|&n| n.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}

fn update_valid(rules: &Vec<(u32, u32)>, update: &Vec<u32>) -> Option<u32> {
    for rule in rules {
        if update.contains(&rule.0) && update.contains(&rule.1) {
            if update.iter().position(|n| n == &rule.0) > update.iter().position(|n| n == &rule.1) {
                return None;
            }
        }
    }
    let midpoint = update.len().sub(1).div(2);
    Some(update[midpoint])
}

fn sum_valid_updates(rules: Vec<(u32, u32)>, updates: Vec<Vec<u32>>) -> u32 {
    let mut sum: u32 = 0;
    for update in updates {
        match fix_updates(&rules, update) {
            None => {},
            Some(midpoint) => sum += midpoint,
        }
    }

    sum
}

fn fix_updates(rules: &Vec<(u32, u32)>, update: Vec<u32>) -> Option<u32> {
    let mut fixed_update = update.clone();
    while update_valid(rules, &fixed_update).is_none() {
        for rule in rules {
            if fixed_update.contains(&rule.0) && fixed_update.contains(&rule.1) {
                let first = fixed_update.iter().position(|n| n == &rule.0).unwrap();
                let second = fixed_update.iter().position(|n| n == &rule.1).unwrap();
                if first > second {
                    fixed_update.swap(first, second);
                }
            }
        }
    }
    if fixed_update == update {
        return None
    };
    let midpoint = fixed_update.len().sub(1).div(2);
    Some(fixed_update[midpoint])
}