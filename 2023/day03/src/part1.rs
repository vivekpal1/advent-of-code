use std::fs;

pub fn run() {
    let input = fs::read_to_string("src/input.txt")
        .expect("Failed to read input file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let total_sum = sum_part_numbers(&grid);
    println!("Total sum of part numbers: {}", total_sum);
}

fn sum_part_numbers(grid: &[Vec<char>]) -> u32 {
    let mut total_sum = 0;
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell.is_digit(10) && is_adjacent_to_symbol(x, y, grid, &directions) {
                total_sum += cell.to_digit(10).unwrap();
            }
        }
    }

    total_sum
}

fn is_adjacent_to_symbol(x: usize, y: usize, grid: &[Vec<char>], directions: &[(i32, i32)]) -> bool {
    directions.iter().any(|&(dx, dy)| {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        nx >= 0 && ny >= 0 && nx < grid[0].len() as i32 && ny < grid.len() as i32 && 
        !grid[ny as usize][nx as usize].is_digit(10) && grid[ny as usize][nx as usize] != '.'
    })
}
