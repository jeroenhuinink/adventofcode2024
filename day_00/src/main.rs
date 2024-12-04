use std::fs::File;
use std::io::{self, BufRead};

fn process_a(lines: Vec<String>) -> i32 {
    let mut total = 0;

    total
}

fn process_b(lines: Vec<String>) -> i32 {
    let mut total = 0;

    total
}

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

fn a() {
    let sample_input = "
";
    let expected = -1;

    let sample_lines = sample_input
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let actual = process_a(sample_lines);

    assert_eq!(actual, expected);

    println!("Test ok");

    let lines = read_file("input.txt");

    let result = process_a(lines);
    println!("===> A ===> {}", result);
}

fn b() {
    let sample_input = "
";
    let expected = -1;

    let sample_lines = sample_input
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let actual = process_b(sample_lines);

    assert_eq!(actual, expected);

    let lines = read_file("input.txt");

    let result = process_b(lines);
    println!("===> B ===> {}", result);
}

fn main() {
    a();
    b();
}
