use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read file");
    let mut res = sort_input(input);
    // let answer = pair_delta_sum(&mut res.0, &mut res.1);
    let answer = similarity_score(&mut res.0, &mut res.1);

    println!("Delta Sum: {}", answer)
}

fn sort_input(input: String) -> (Vec<u64>, Vec<u64>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let lines: Vec<&str> = input.split("\n").collect();
    let pairs: Vec<Vec<&str>> = lines.iter().map(|&line| line.split("   ").collect()).collect();

    for pair in pairs {
        left.push(pair[0].parse::<u64>().unwrap());
        right.push(pair[1].parse::<u64>().unwrap());
    }

    (left, right)
}

fn pair_delta_sum(left: &mut Vec<u64>, right: &mut Vec<u64>) -> u64 {
    let size = left.len();
    let mut count = 0;
    let mut sum = 0;
    while count < size {
        let min_left = left.iter().min().unwrap().clone();
        left.remove(left.iter().position(|&num| num == min_left).unwrap());
        let min_right = right.iter().min().unwrap().clone();
        right.remove(right.iter().position(|&num| num == min_right).unwrap());

        sum += min_left.abs_diff(min_right);

        count += 1;
    }

    sum
}

fn similarity_score(left: &mut Vec<u64>, right: &mut Vec<u64>) -> u64 {
    let mut sum: u64 = 0;
    for left_num in left {
        let right_count: u64 = right.iter().filter(|&&x| x == *left_num).count().try_into().unwrap();
        sum += *left_num * right_count
    }

    sum
}