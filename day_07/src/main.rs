use std::{collections::BTreeMap, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read file");
    let equations = parse_input(input);
    println!("Equations: {:?}", equations);
    let answer = sum_true_equations(equations);
    println!("Answer: {}", answer);
}

fn parse_input(input: String) -> BTreeMap<u128, Vec<u128>> {
    input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&line| {
            let (test_val, equation_inputs) = line.split_once(":").unwrap();
            (
                test_val.parse::<u128>().unwrap(),
                equation_inputs
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|&num| num.parse::<u128>().unwrap())
                    .collect::<Vec<u128>>(),
            )
        })
        .collect::<BTreeMap<u128, Vec<u128>>>()
}

fn sum_true_equations(equations: BTreeMap<u128, Vec<u128>>) -> u128 {
    let mut total: u128 = 0;
    for (test_val, mut nums) in equations {
        println!("Checking Test Value: {} with numbers: {:?}", test_val, nums);
        let missing_operators: u32 = (nums.len() - 1).try_into().unwrap();
        println!("Missing Operators: {}", missing_operators);
        let permutations = u32::pow(3, missing_operators);
        let mut unique_op_sequences: Vec<Vec<Operator>> =
            vec![vec![Operator::Add], vec![Operator::Multiply], vec![Operator::Combine]];
        println!("Range: {:?}", 1..missing_operators);
        for _ in 1..missing_operators {
            let mut new_sequences: Vec<Vec<Operator>> = vec![];
            for sequence in &mut unique_op_sequences {
                let mut new_add: Vec<Operator> = sequence.clone();
                let mut new_mult: Vec<Operator> = sequence.clone();
                let mut new_combine: Vec<Operator> = sequence.clone();
                new_add.push(Operator::Add);
                new_mult.push(Operator::Multiply);
                new_combine.push(Operator::Combine);
                new_sequences.push(new_add);
                new_sequences.push(new_mult);
                new_sequences.push(new_combine);
            }
            unique_op_sequences = new_sequences.clone();
        }
        assert!(unique_op_sequences.len() == permutations.try_into().unwrap());
        let first_num = nums.first().expect("Equations Vector is empty").to_owned();
        nums.remove(0);
        for sequence in &unique_op_sequences {
            let zipped: Vec<(&Operator, &u128)> = sequence.iter().zip(&nums).collect();
            if do_math(first_num, zipped) == test_val {
                total += test_val;
                break;
            }
        }
    }

    total
}

#[derive(Clone, Debug, Copy)]
enum Operator {
    Add,
    Multiply,
    Combine,
}

fn do_math(starting_val: u128, equation_inputs: Vec<(&Operator, &u128)>) -> u128 {
    equation_inputs
        .iter()
        .fold(starting_val, |acc, pair| match &pair.0 {
            Operator::Add => acc + pair.1,
            Operator::Multiply => acc * pair.1,
            Operator::Combine => {
                let mut combined = acc.to_string();
                combined.push_str(&pair.1.to_string());
                combined.parse::<u128>().expect("Failed to parse string to u128")
            }
        })
}
