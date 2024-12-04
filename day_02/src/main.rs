use std::fs::File;
use std::io::{self, BufRead};

fn read_file(path: &str) -> Vec<String> {
    // Open the file in read-only mode
    let file = File::open(path).expect("Could not open file");

    // Create a buffered reader
    let reader = io::BufReader::new(file);

    // Read lines into a vector
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not read line"))
        .filter(|s| !s.is_empty())
        .collect();

    lines
}

fn is_increasing(arr: &Vec<i32>) -> bool {
    arr.iter().zip(arr.iter().skip(1)).all(|(a, b)| a < b)
}

fn is_decreasing(arr: &Vec<i32>) -> bool {
    arr.iter().zip(arr.iter().skip(1)).all(|(a, b)| a > b)
}

fn are_in_range(arr: &Vec<i32>) -> bool {
    arr.iter()
        .zip(arr.iter().skip(1))
        .all(|(a, b)| (a - b).abs() >= 1 && (a - b).abs() <= 3)
}

fn process_a(lines: Vec<String>) -> i32 {
    let mut safe = 0;
    let reports: Vec<Vec<i32>> = lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>())
                .filter_map(Result::ok)
                .collect()
        })
        .collect();
    for report in reports {
        if (is_increasing(&report) || is_decreasing(&report)) && are_in_range(&report) {
            safe = safe + 1
        }
    }
    safe
}

fn process_b(lines: Vec<String>) -> i32 {
    let mut safe = 0;
    let reports: Vec<Vec<i32>> = lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>())
                .filter_map(Result::ok)
                .collect()
        })
        .collect();
    for report in reports {
        if (is_increasing(&report) || is_decreasing(&report)) && are_in_range(&report) {
            safe = safe + 1;
            continue;
        }
        for i in 0..report.len() {
            let mut r = report.clone();

            r.remove(i);

            if (is_increasing(&r) || is_decreasing(&r)) && are_in_range(&r) {
                safe = safe + 1;
                break;
            }
        }
    }
    safe
}

fn a() {
    let sample_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    let expected = 2;

    let sample_lines = sample_input
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let actual = process_a(sample_lines);

    assert_eq!(actual, expected);

    let lines = read_file("input.txt");

    let result = process_a(lines);
    println!("A: {}", result);
}

fn b() {
    let sample_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    let expected = 4;

    let sample_lines = sample_input
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let actual = process_b(sample_lines);

    assert_eq!(actual, expected);

    let lines = read_file("input.txt");

    let result = process_b(lines);
    println!("B: {}", result);
}
fn main() {
    a();
    b();
}
