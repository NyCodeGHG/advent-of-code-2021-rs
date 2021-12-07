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
    let (fuel, pos) =
        calculate_fuel(&crabs, |crab, pos| i32::abs(crab - pos)).expect("No solution found");
    println!("Position: {}, Fuel: {}", pos, fuel);
    let (fuel, pos) = calculate_fuel(&crabs, fuel_steps).expect("No solution found");
    println!("Position: {}, Fuel: {}", pos, fuel);
    println!("Solution took {}ms", now.elapsed().unwrap().as_millis());
}

fn calculate_fuel<F>(crabs: &[i32], fuel_algorithm: F) -> Option<(i32, i32)>
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
            (
                crabs
                    .iter()
                    .map(|crab| fuel_algorithm(*crab, *pos))
                    .sum::<i32>(),
                *pos,
            )
        })
        .min()
}

fn fuel_steps(crab: i32, position: i32) -> i32 {
    let distance = i32::abs(crab - position);
    (1..=distance).sum()
}
