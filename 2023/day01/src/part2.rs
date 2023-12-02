use std::fs;

pub fn run() {
    let input = fs::read_to_string("./src/input.txt")
        .expect("Failed to read input file");

    let sum = process(&input);
    println!("Total sum of calibration values (Part 2): {}", sum);
}

fn process(input: &str) -> String {
    input.lines()
        .map(process_line)
        .sum::<u32>()
        .to_string()
}

fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = match reduced_line {
            _ if reduced_line.starts_with("one") => '1',
            _ if reduced_line.starts_with("two") => '2',
            _ if reduced_line.starts_with("three") => '3',
            _ if reduced_line.starts_with("four") => '4',
            _ if reduced_line.starts_with("five") => '5',
            _ if reduced_line.starts_with("six") => '6',
            _ if reduced_line.starts_with("seven") => '7',
            _ if reduced_line.starts_with("eight") => '8',
            _ if reduced_line.starts_with("nine") => '9',
            _ => reduced_line.chars().next().unwrap_or('0'),
        };
        result.to_digit(10)
    });

    let first = it.next().unwrap_or(0);
    let last = it.last().unwrap_or(first);

    first * 10 + last
}
