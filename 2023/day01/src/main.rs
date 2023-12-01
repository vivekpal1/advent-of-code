use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        fs::read_to_string(&args[1]).expect("Failed to read input file")
    } else {
        // Default input
        String::from("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet")
    };

    let sum = calculate_sum(&input);
    println!("Total sum of calibration values: {}", sum);
}

fn calculate_sum(input: &str) -> i32 {
    let mut total_sum = 0;
    for line in input.lines() {
        if let Some(first_digit) = line.chars().find(|c| c.is_digit(10)) {
            if let Some(last_digit) = line.chars().rev().find(|c| c.is_digit(10)) {
                let value = format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap();
                total_sum += value;
            }
        }
    }
    total_sum
}
