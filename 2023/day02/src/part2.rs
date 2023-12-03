use std::fs;
use std::collections::HashMap;

pub fn run() {
    let input = fs::read_to_string("./src/input.txt")
        .expect("Failed to read input file");
    let sum_of_powers = process(&input);
    println!("Sum of the powers of the sets: {}", sum_of_powers);
}

fn process(input: &str) -> u32 {
    input.lines()
        .map(|line| parse_game(line))
        .map(|game| game.minimum_cube_set_power())
        .sum()
}

fn parse_game(line: &str) -> Game {
    let parts: Vec<&str> = line.split(": ").collect();
    let id = parts[0];
    let rounds = parts[1].split("; ")
        .map(|round| {
            round.split(", ").map(|cube| {
                let parts: Vec<&str> = cube.split_whitespace().collect();
                (parts[1], parts[0].parse::<u32>().unwrap())
            }).collect::<HashMap<_, _>>()
        })
        .collect();
    Game { id, rounds }
}

struct Game<'a> {
    id: &'a str,
    rounds: Vec<HashMap<&'a str, u32>>,
}

impl<'a> Game<'a> {
    fn minimum_cube_set_power(&self) -> u32 {
        let mut max_cubes: HashMap<&str, u32> = HashMap::new();
        for round in &self.rounds {
            for (&color, &amount) in round {
                max_cubes.entry(color).and_modify(|e| *e = std::cmp::max(*e, amount)).or_insert(amount);
            }
        }
        max_cubes.values().product()
    }
}
