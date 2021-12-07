use std::fs;
use itertools::Itertools;

fn main() {
    let crabs: Vec<i32> = fs::read_to_string("inputs/day07.txt")
        .expect("Unable to read input")
        .lines()
        .next()
        .unwrap()
        .split(',')
        .filter_map(|v| v.parse::<i32>().ok())
        .collect();
    let (fuel, pos) = calculate_fuel(&crabs).expect("No solution found");
    println!("Position: {}, Fuel: {}", pos, fuel);
}

fn calculate_fuel(crabs: &[i32]) -> Option<(i32, i32)> {
    let positions = crabs.iter().sorted().dedup().collect::<Vec<&i32>>();
    positions.iter().map(|pos| {
        (crabs.iter().map(|crab| i32::abs(crab - **pos)).sum::<i32>(), **pos)
    }).min()
}
