use std::fs;
use std::collections::HashMap;

pub fn run() {
    let input = fs::read_to_string("./src/input.txt")
        .expect("Failed to read input file");
    let sum_of_ids = process(&input);
    println!("Sum of IDs of feasible games: {}", sum_of_ids);
}

fn process(input: &str) -> i32 {
    input.lines()
        .map(|line| parse_game(line))
        .filter(|game| is_game_feasible(game))
        .map(|game| game.0)
        .sum()
}

fn parse_game(line: &str) -> (i32, Vec<HashMap<&str, i32>>) {
    let parts: Vec<&str> = line.split(": ").collect();
    let game_id = parts[0].split_whitespace().last().unwrap().parse::<i32>().unwrap();
    let rounds = parts[1].split("; ")
        .map(|round| {
            round.split(", ").map(|cube| {
                let parts: Vec<&str> = cube.split_whitespace().collect();
                (parts[1], parts[0].parse::<i32>().unwrap())
            }).collect::<HashMap<&str, i32>>()
        }).collect();
    (game_id, rounds)
}

fn is_game_feasible(game: &(i32, Vec<HashMap<&str, i32>>)) -> bool {
    let cube_limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    game.1.iter().all(|round| {
        round.iter().all(|(color, &count)| {
            *cube_limits.get(*color).unwrap_or(&0) >= count
        })
    })
}
