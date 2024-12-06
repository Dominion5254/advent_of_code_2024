use std::{fs, ops::{Add, Sub}};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read file");
    let diagram = parse_input(input);
    let answer = count_distinct_positions(diagram);

    println!("Answer: {}", answer);
}

enum Orientation {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    return input.split("\n").collect::<Vec<&str>>()
        .iter()
        .map(|&row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
}

fn count_distinct_positions(mut diagram: Vec<Vec<char>>) -> usize {
    let row_count = diagram.len();
    let col_count = diagram[0].len();
    let mut guard_orientation = Orientation::UP;
    let initial_y = diagram.iter().position(|row| row.contains(&'^')).unwrap();
    let initial_x = diagram[initial_y].iter().position(|&c| c == '^').unwrap();
    let mut guard_position = (initial_y, initial_x);
    diagram[guard_position.0][guard_position.1] = '.';
    let mut distinct_positions: Vec<(usize, usize)> = vec![];
    
    loop {
        if guard_position.0 == 0 || guard_position.0 == row_count.sub(1) {
            break
        } else if guard_position.1 == 0 || guard_position.1 == col_count.sub(1) {
            break
        }

        match guard_orientation {
            Orientation::UP => { 
                let next_forward = (guard_position.0.sub(1), guard_position.1);
                if !forward_safe(next_forward, &mut guard_position, &diagram, &mut distinct_positions) {
                    guard_position = (guard_position.0, guard_position.1.add(1));
                    guard_orientation = Orientation::RIGHT;
                    if !distinct_positions.contains(&guard_position) {
                        distinct_positions.push(guard_position);
                    }
                }
            }
            Orientation::DOWN => { 
                let next_forward = (guard_position.0.add(1), guard_position.1);
                if !forward_safe(next_forward, &mut guard_position, &diagram, &mut distinct_positions) {
                    guard_position = (guard_position.0, guard_position.1.sub(1));
                    guard_orientation = Orientation::LEFT;
                    if !distinct_positions.contains(&guard_position) {
                        distinct_positions.push(guard_position);
                    }
                }
            }
            Orientation::LEFT => { 
                let next_forward = (guard_position.0, guard_position.1.sub(1));
                if !forward_safe(next_forward, &mut guard_position, &diagram, &mut distinct_positions) {
                    guard_position = (guard_position.0.sub(1), guard_position.1);
                    guard_orientation = Orientation::UP;
                    if !distinct_positions.contains(&guard_position) {
                        distinct_positions.push(guard_position);
                    }
                }
            }
            Orientation::RIGHT => { 
                let next_forward = (guard_position.0, guard_position.1.add(1));
                if !forward_safe(next_forward, &mut guard_position, &diagram, &mut distinct_positions) {
                    guard_position = (guard_position.0.add(1), guard_position.1);
                    guard_orientation = Orientation::DOWN;
                    if !distinct_positions.contains(&guard_position) {
                        distinct_positions.push(guard_position);
                    }
                }
            }
        }
    }
    distinct_positions.len()
}

fn forward_safe(next_forward: (usize, usize), guard_position: &mut (usize, usize), diagram: &Vec<Vec<char>>, distinct_positions: &mut Vec<(usize, usize)>) -> bool {
    if diagram[next_forward.0][next_forward.1] == '.' {
        guard_position.0 = next_forward.0;
        guard_position.1 = next_forward.1;
        if !distinct_positions.contains(&guard_position) {
            distinct_positions.push(*guard_position);
        }
        return true;
    }

    false
}