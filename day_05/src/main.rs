use std::fs::File;
use std::io::{self, BufRead};

fn matches_rules(update: &Vec<i32>, rules: &Vec<Vec<i32>>) -> bool {
    if update.len() == 0 {
        return true;
    }

    for i in 1..update.len() {
        let mut applicable_rules = rules.iter().filter(|rule| rule[0] == update[0]);
        if !applicable_rules.any(|rule| rule[1] == update[i]) {
            return false;
        }
    }

    matches_rules(&update[1..].to_vec(), rules)
}

fn process_a(lines: Vec<String>) -> i32 {
    let mut rules: Vec<Vec<i32>> = Vec::new();
    let mut i = 0;
    while lines[i].len() > 0 {
        let rule: Vec<i32> = lines[i]
            .split("|")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        rules.push(rule);
        i = i + 1;
    }

    i = i + 1;
    let mut updates: Vec<Vec<i32>> = Vec::new();
    while i < lines.len() && lines[i].len() > 0 {
        let update: Vec<i32> = lines[i]
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        updates.push(update);
        i = i + 1;
    }

    let mut total = 0;
    for update in updates {
        if matches_rules(&update, &rules) {
            total = total + update[update.len() / 2];
        }
    }
    total
}

fn compare_with_rules(a: &i32, b: &i32, rules: &Vec<Vec<i32>>) -> std::cmp::Ordering {
    let mut applicable_rules = rules.iter().filter(|rule| rule[0] == *a);
    if !applicable_rules.any(|rule| rule[1] == *b) {
        return std::cmp::Ordering::Greater;
    }

    return std::cmp::Ordering::Less;
}

fn process_b(lines: Vec<String>) -> i32 {
    let mut rules: Vec<Vec<i32>> = Vec::new();
    let mut i = 0;
    while lines[i].len() > 0 {
        let rule: Vec<i32> = lines[i]
            .split("|")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        rules.push(rule);
        i = i + 1;
    }

    i = i + 1;
    let mut updates: Vec<Vec<i32>> = Vec::new();
    while i < lines.len() && lines[i].len() > 0 {
        let update: Vec<i32> = lines[i]
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        updates.push(update);
        i = i + 1;
    }

    let mut total = 0;
    for mut update in updates {
        if !matches_rules(&update, &rules) {
            update.sort_by(|a, b| compare_with_rules(a, b, &rules));
            total = total + update[update.len() / 2];
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
        .collect();

    lines
}

fn a() {
    let sample_input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    let expected = 143;

    let sample_lines = sample_input.split("\n").map(|s| s.to_string()).collect();

    let actual = process_a(sample_lines);

    assert_eq!(actual, expected);

    println!("Test ok");

    let lines = read_file("input.txt");

    let result = process_a(lines);
    println!("===> A ===> {}", result);
}

fn b() {
    let sample_input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    let expected = 123;

    let sample_lines = sample_input.split("\n").map(|s| s.to_string()).collect();

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
