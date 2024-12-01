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

fn process_a(lines: Vec<String>) -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = lines
        .into_iter()
        .map(|line| {
            let split: Vec<&str> = line.split("   ").collect();
            (
                split[0].parse::<i32>().unwrap(),
                split[1].parse::<i32>().unwrap(),
            )
        })
        .clone()
        .unzip();
    left.sort();
    right.sort();
    let mut total = 0;
    for (index, _) in left.iter().enumerate() {
        let diff = left[index] - right[index];
        total = total + diff.abs();
    }
    total
}

fn process_b(lines: Vec<String>) -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = lines
        .into_iter()
        .map(|line| {
            let split: Vec<&str> = line.split("   ").collect();
            (
                split[0].parse::<i32>().unwrap(),
                split[1].parse::<i32>().unwrap(),
            )
        })
        .clone()
        .unzip();
    left.sort();
    right.sort();
    let mut total = 0;
    for (index, _) in left.iter().enumerate() {
        let count: i32 = right
            .iter()
            .filter(|&&x| x == left[index])
            .count()
            .try_into()
            .unwrap();

        total = total + (left[index] * count);
    }
    total
}

fn a() {
    let sample_input = "3   4
4   3
2   5
1   3
3   9
3   3
";
    let expected = 11;

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
    let sample_input = "3   4
4   3
2   5
1   3
3   9
3   3
";
    let expected = 31;

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
