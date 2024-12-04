use std::fs::File;
use std::io::{self, BufRead};

// M.S
// .A.
// M.S
// M.M
// .A.
// S.S
// S.M
// .A.
// S.M
// S.S
// .A.
// M.M

fn check2_w(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let line = &chars[i];
    if j < 3 {
        false
    } else {
        line[j] == 'X' && line[j - 1] == 'M' && line[j - 2] == 'A' && line[j - 3] == 'S'
    }
}

fn check2_e(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let line = &chars[i];
    if j >= line.len() - 3 {
        false
    } else {
        line[j] == 'X' && line[j + 1] == 'M' && line[j + 2] == 'A' && line[j + 3] == 'S'
    }
}

fn checkb_var1(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i < 1 || j < 1 || i >= chars.len() - 1 || j >= chars[i].len() - 1 {
        false
    } else {
        chars[i - 1][j - 1] == 'M'
            && chars[i + 1][j - 1] == 'M'
            && chars[i - 1][j + 1] == 'S'
            && chars[i + 1][j + 1] == 'S'
    }
}

fn checkb_var2(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i < 1 || j < 1 || i >= chars.len() - 1 || j >= chars[i].len() - 1 {
        false
    } else {
        chars[i - 1][j - 1] == 'S'
            && chars[i + 1][j - 1] == 'S'
            && chars[i - 1][j + 1] == 'M'
            && chars[i + 1][j + 1] == 'M'
    }
}

fn checkb_var3(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i < 1 || j < 1 || i >= chars.len() - 1 || j >= chars[i].len() - 1 {
        false
    } else {
        chars[i - 1][j - 1] == 'S'
            && chars[i + 1][j - 1] == 'M'
            && chars[i - 1][j + 1] == 'S'
            && chars[i + 1][j + 1] == 'M'
    }
}

fn checkb_var4(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i < 1 || j < 1 || i >= chars.len() - 1 || j >= chars[i].len() - 1 {
        false
    } else {
        chars[i - 1][j - 1] == 'M'
            && chars[i + 1][j - 1] == 'S'
            && chars[i - 1][j + 1] == 'M'
            && chars[i + 1][j + 1] == 'S'
    }
}

fn check_w(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let line = &chars[i];
    if j < 3 {
        false
    } else {
        line[j] == 'X' && line[j - 1] == 'M' && line[j - 2] == 'A' && line[j - 3] == 'S'
    }
}

fn check_e(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let line = &chars[i];
    if j >= line.len() - 3 {
        false
    } else {
        line[j] == 'X' && line[j + 1] == 'M' && line[j + 2] == 'A' && line[j + 3] == 'S'
    }
}

fn check_n(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i < 3 {
        false
    } else {
        chars[i][j] == 'X'
            && chars[i - 1][j] == 'M'
            && chars[i - 2][j] == 'A'
            && chars[i - 3][j] == 'S'
    }
}

fn check_s(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i >= chars.len() - 3 {
        false
    } else {
        chars[i][j] == 'X'
            && chars[i + 1][j] == 'M'
            && chars[i + 2][j] == 'A'
            && chars[i + 3][j] == 'S'
    }
}

fn check_ne(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i < 3 || j < 3 {
        false
    } else {
        chars[i][j] == 'X'
            && chars[i - 1][j - 1] == 'M'
            && chars[i - 2][j - 2] == 'A'
            && chars[i - 3][j - 3] == 'S'
    }
}

fn check_nw(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i < 3 || j >= chars[i].len() - 3 {
        false
    } else {
        chars[i][j] == 'X'
            && chars[i - 1][j + 1] == 'M'
            && chars[i - 2][j + 2] == 'A'
            && chars[i - 3][j + 3] == 'S'
    }
}

fn check_se(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i >= chars.len() - 3 || j < 3 {
        false
    } else {
        chars[i][j] == 'X'
            && chars[i + 1][j - 1] == 'M'
            && chars[i + 2][j - 2] == 'A'
            && chars[i + 3][j - 3] == 'S'
    }
}

fn check_sw(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i >= chars.len() - 3 || j >= chars[i].len() - 3 {
        false
    } else {
        chars[i][j] == 'X'
            && chars[i + 1][j + 1] == 'M'
            && chars[i + 2][j + 2] == 'A'
            && chars[i + 3][j + 3] == 'S'
    }
}

fn process_a(lines: Vec<String>) -> i32 {
    let mut total = 0;
    let chars: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            if chars[i][j] == 'X' {
                if check_n(&chars, i, j) {
                    total += 1;
                }
                if check_e(&chars, i, j) {
                    total += 1;
                }
                if check_s(&chars, i, j) {
                    total += 1;
                }
                if check_w(&chars, i, j) {
                    total += 1;
                }
                if check_ne(&chars, i, j) {
                    total += 1;
                }
                if check_nw(&chars, i, j) {
                    total += 1;
                }
                if check_se(&chars, i, j) {
                    total += 1;
                }
                if check_sw(&chars, i, j) {
                    total += 1;
                }
            }
        }
    }
    total
}

fn process_b(lines: Vec<String>) -> i32 {
    let mut total = 0;
    let chars: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            if chars[i][j] == 'A' {
                if checkb_var1(&chars, i, j) {
                    total += 1;
                }
                if checkb_var2(&chars, i, j) {
                    total += 1;
                }
                if checkb_var3(&chars, i, j) {
                    total += 1;
                }
                if checkb_var4(&chars, i, j) {
                    total += 1;
                }
            }
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

fn parse_lines(input: &str) -> Vec<String> {
    input
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn a() {
    let sample_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    let expected = 18;

    let sample_lines = parse_lines(sample_input);

    let actual = process_a(sample_lines);

    assert_eq!(actual, expected);

    println!("Test ok");

    let lines = read_file("input.txt");

    let result = process_a(lines);
    println!("===> A ===> {}", result);
}

fn b() {
    let sample_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    let expected = 9;

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
