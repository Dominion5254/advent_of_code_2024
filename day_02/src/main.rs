use std::fs;

fn main() {
    let input = fs::read_to_string("./example.txt").expect("Failed to read file");
    let reports = parse_input(input);
    let safe_count = count_reports(reports);
    println!("Safe Reports: {}", safe_count);
}

fn parse_input(input: String) -> Vec<Vec<u64>> {
    let lines: Vec<&str> = input.split("\n").collect();
    let reports = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|level| level.parse::<u64>().unwrap())
                .collect()
        })
        .collect();
    reports
}

fn report_safe(report: Vec<u64>) -> bool {
    let increasing = report[1] > report[0];
    let mut problem_dampener = 0;
    for i in 0..(report.len() - 1) {
        // if i <= report.len() - 3 {
        //     if (increasing && report[i + 1] < report[i]) || (!increasing && report[i + 1] > report[i]) {
        //         // println!("Checking index {} of {} with a value of {}", i, report.len() -1, report[i]);
        //         // println!("Report: {:?}", report);
        //         if (increasing && report[i + 2] < report[i]) || (!increasing && report[i + 2] > report[i]) {
        //             return false;
        //         }
        //         problem_dampener += 1;
        //     }
        //     match report[i].abs_diff(report[i + 1]) {
        //         3 => {},
        //         2 => {},
        //         1 => {},
        //         _ => {
        //             match report[i].abs_diff(report[i + 2]) {
        //                 3 => {},
        //                 2 => {},
        //                 1 => {},
        //                 _ => return false,
        //             }
        //             problem_dampener += 1;
        //         },
        //     }
        //     if problem_dampener > 1 {
        //         return false;
        //     }
        // } else {
            if (increasing && report[i + 1] < report[i]) || (!increasing && report[i + 1] > report[i]) {
                return false;
            }
            match report[i].abs_diff(report[i + 1]) {
                3 => {},
                2 => {},
                1 => {},
                _ => {
                    return false;
                },
            }
        // }
    }
    true
}

// fn report_safe(report: Vec<u64>) -> bool {
//     let spare = report.clone();
//     let mut iterator = report.windows(3);
//     let pair = iterator.next().unwrap();
//     let increasing = pair[0] < pair[1];
//     let mut problem_dampener = 0;
//     if pair[0].abs_diff(pair[1]) > 3 || pair[0].abs_diff(pair[1]) < 1 {
//         if pair[0].abs_diff(pair[2]) > 3 || pair[0].abs_diff(pair[2]) < 1 {
//             return false;
//         }
//         problem_dampener += 1;
//     }

//     loop {
//         let pair = iterator.next();
//         match pair {
//             None => {
//                 let new_iterator = spare.windows(2);
//                 match new_iterator.last() {
//                     None => return true,
//                     Some(pair) => {
//                         if pair[0] > pair[1] && increasing {
//                             problem_dampener += 1;
//                             // return false;
//                         }
//                         if !increasing && pair[1] > pair[0] {
//                             problem_dampener += 1;
//                             // return false;
//                         }
//                         match pair[0].abs_diff(pair[1]) {
//                             3 => {},
//                             2 => {},
//                             1 => {},
//                             _ => {
//                                 // return false;
//                                 problem_dampener += 1;
//                             }
//                         }
//                         if problem_dampener > 1 {
//                             return false;
//                         } else {
//                             return true;
//                         }
//                     }
//                 }
//             }
//             Some(pair) => {
//                 if pair[0] > pair[1] && increasing {
//                     if pair[0] > pair[2] && increasing {
//                         return false;
//                     }
//                     problem_dampener += 1;
//                 }
//                 if !increasing && pair[1] > pair[0] {
//                     if !increasing && pair[2] > pair[0] {
//                         return false;
//                     }
//                     problem_dampener += 1;
//                 }
//                 match pair[0].abs_diff(pair[1]) {
//                     3 => {},
//                     2 => {},
//                     1 => {},
//                     _ => {
//                         if pair[0].abs_diff(pair[2]) > 3 || pair[0].abs_diff(pair[2]) < 1 {
//                             return false;
//                         }
//                         problem_dampener += 1;
//                     }
//                 }
//                 if problem_dampener > 1 {
//                     return false;
//                 }
//             }
//         }
//     }
// }

fn count_reports(reports: Vec<Vec<u64>>) -> u64 {
    let mut count = 0;
    for report in reports {
        if report_safe(report.clone()) {
            count += 1;
            println!("Report: {:?}", report)
        }
    }
    count
}
