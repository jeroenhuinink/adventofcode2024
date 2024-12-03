use std::fs::File;
use std::io::{self, BufRead};

fn process_a(lines: Vec<String>) -> i32 {
    let mut total = 0;
    for line in lines {
        println!("{}", line);
        let re = regex::Regex::new(r"mul\((?<left>\d+),(?<right>\d+)\)").unwrap();
        let captures = re.captures_iter(&line);

        for (i, cap) in captures.enumerate() {
            let left = &cap["left"].parse::<i32>().unwrap();
            let right = &cap["right"].parse::<i32>().unwrap();
            total = total + (left * right);
            println!("capture {}: {:?}*{:?}", i, left, right);
        }
    }
    total
}

fn process_b(lines: Vec<String>) -> i32 {
    let mut total = 0;
    let mut skip = false;
    for line in lines {
        println!("{}", line);
        let re =
            regex::Regex::new(r"(mul\((?<left>\d+),(?<right>\d+)\)|do\(\)|don't\(\))").unwrap();
        let captures = re.captures_iter(&line);

        for (i, cap) in captures.enumerate() {
            if &cap[0] == "do()" {
                skip = false;
                continue;
            }
            if &cap[0] == "don't()" {
                skip = true;
                continue;
            }
            let left = &cap["left"].parse::<i32>().unwrap();
            let right = &cap["right"].parse::<i32>().unwrap();
            if !skip {
                total = total + (left * right);
            }
            println!("capture {}: {:?}*{:?}", i, left, right);
        }
    }
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
    let sample_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+
mul(32,64]then(mul(11,8)mul(8,5))
";
    let expected = 161;

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
    let sample_input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";
    let expected = 48;

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
