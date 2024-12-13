use std::{collections::HashMap, fs, ops::{Add, Sub}};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read file");
    let (grid, antenna_map) = parse_input(input);
    let answer = count_antinodes(grid, antenna_map);
    println!("Answer: {}", answer);
}

fn parse_input(input: String) -> (Vec<Vec<char>>, HashMap<char, Vec<(i32, i32)>>) {
    let grid = input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut antenna_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let row_count = grid.len();
    let col_count = grid[0].len();

    for row_idx in 0..row_count {
        for col_idx in 0..col_count {
            let current_char: char = grid[row_idx][col_idx];
            if current_char != '.' {
                if antenna_map.contains_key(&current_char) {
                    antenna_map
                        .get_mut(&current_char)
                        .unwrap()
                        .push((row_idx as i32, col_idx as i32));
                } else {
                    antenna_map.insert(current_char, vec![(row_idx as i32, col_idx as i32)]);
                }
            }
        }
    }

    (grid, antenna_map)
}

fn count_antinodes(grid: Vec<Vec<char>>, antenna_map: HashMap<char, Vec<(i32, i32)>>) -> usize {
    let mut antinode_locations: Vec<(i32, i32)> = vec![];
    for (_antenna, locations) in antenna_map {
        let mut antenna_pairs: Vec<((i32, i32), (i32, i32))> = vec![];
        for location in &locations {
            for other_location in
                locations.iter().position(|&e| e == *location).unwrap().add(1)..locations.len()
            {
                antenna_pairs.push((*location, locations[other_location]));
            }
        }
        for (pair1, pair2) in antenna_pairs {
            // (0, 0)
            // (2, 1)
            let y_delta = pair1.0.abs_diff(pair2.0) as i32; // 2
            // let x_delta = pair1.1.abs_diff(pair2.1) as i32; // 1
            let x_slope = pair2.1.sub(pair1.1) as i32; // 1
            // let mut antinodes: Vec<(i32, i32)> = vec![];
            for pair in vec![pair1, pair2] {
                if !antinode_locations.contains(&pair) {
                    antinode_locations.push(pair);
                }
            }
            let mut up = (pair1.0, pair1.1);
            loop {
                up.0 = up.0.sub(y_delta);
                up.1 = up.1.sub(x_slope);
                // println!("testing up: {:?}", up);
                match grid.get(up.0 as usize) {
                    None => { break },
                    Some(_) => {
                        match grid[up.0 as usize].get(up.1 as usize) {
                            None => { break },
                            Some(_) => {
                                if !antinode_locations.contains(&up) {
                                    antinode_locations.push(up);
                                }
                            }
                        }
                    }
                }
            }

            let mut down = (pair2.0, pair2.1);
            loop {
                down.0 = down.0.add(y_delta);
                down.1 = down.1.add(x_slope);
                match grid.get(down.0 as usize) {
                    None => { break },
                    Some(_) => {
                        match grid[down.0 as usize].get(down.1 as usize) {
                            None => { break },
                            Some(_) => {
                                if !antinode_locations.contains(&down) {
                                    antinode_locations.push(down);
                                }
                            }
                        }
                    }
                }
            }
            
            // let antinode1x: i32; 
            // let antinode2x: i32; 
            // if pair1.1 < pair2.1 {
            //     antinode1x = pair1.1.min(pair2.1).sub(x_delta);
            //     antinode2x = pair1.1.max(pair2.1).add(x_delta);
            // } else {
            //     antinode1x = pair1.1.max(pair2.1).add(x_delta);
            //     antinode2x = pair1.1.min(pair2.1).sub(x_delta);
            // }
            // let antinode1 = (pair1.0.min(pair2.0).sub(y_delta), antinode1x); // (1, 3)
            // let antinode2 = (pair1.0.max(pair2.0).add(y_delta), antinode2x); // (7, 9)
            // for antinode in vec![antinode1, antinode2] {
            //     match grid.get(antinode.0 as usize) {
            //         None => {},
            //         Some(_) => {
            //             match grid[antinode.0 as usize].get(antinode.1 as usize) {
            //                 None => {},
            //                 Some(_) => {
            //                     if !antinode_locations.contains(&antinode) {
            //                         antinode_locations.push(antinode);
            //                     }
            //                 }
            //             }
            //         }
            //     }
            // }

        }
    }
    println!("Antinode Locations: {:?}", antinode_locations);

    antinode_locations.len()
}

