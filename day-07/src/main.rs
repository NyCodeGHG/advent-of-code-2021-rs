use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::{fs, time::SystemTime};

fn main() {
    let crabs: Vec<i32> = fs::read_to_string("inputs/day07.txt")
        .expect("Unable to read input")
        .lines()
        .next()
        .unwrap()
        .split(',')
        .filter_map(|v| v.parse::<i32>().ok())
        .collect();
    let now = SystemTime::now();
    let (fuel_one, fuel_two) = get_solution(&crabs);
    println!("Fuel: {}", fuel_one);
    println!("Fuel: {}", fuel_two);
    println!("Solution took {}ms", now.elapsed().unwrap().as_millis());
}

fn get_solution(crabs: &[i32]) -> (i32, i32) {
    (
        calculate_fuel(crabs, |crab, pos| i32::abs(crab - pos)).expect("No solution found"),
        calculate_fuel(crabs, fuel_steps).expect("No solution found"),
    )
}

fn calculate_fuel<F>(crabs: &[i32], fuel_algorithm: F) -> Option<i32>
where
    F: Fn(i32, i32) -> i32,
{
    let positions = match crabs.iter().minmax() {
        MinMax(a, b) => ((*a)..=(*b)).collect_vec(),
        _ => return None,
    };
    positions
        .iter()
        .map(|pos| {
            crabs
                .iter()
                .map(|crab| fuel_algorithm(*crab, *pos))
                .sum::<i32>()
        })
        .min()
}

fn fuel_steps(crab: i32, position: i32) -> i32 {
    let n = i32::abs(crab - position);
    n * (n + 1) / 2
}
