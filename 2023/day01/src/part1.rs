use std::fs;
use std::process;

pub fn run() {
    let input = fs::read_to_string("./src/input.txt").unwrap_or_else(|_| {
        eprintln!("Failed to read input file: ./src/input.txt");
        process::exit(1);
    });

    let sum = calculate_sum(&input);
    println!("Total sum of calibration values (Part 1): {}", sum);
}

pub fn calculate_sum(input: &str) -> i32 {
    input
        .lines()
        .filter_map(extract_and_combine_digits)
        .sum()
}

pub fn extract_and_combine_digits(line: &str) -> Option<i32> {
    let first_digit = line.chars().find(|c| c.is_digit(10))?;
    let last_digit = line.chars().rev().find(|c| c.is_digit(10))?;
    Some(format!("{}{}", first_digit, last_digit).parse().ok()?)
}
