use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    let fishes: Vec<u8> = fs::read_to_string("inputs/day06.txt")
        .expect("Unable to read input")
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse::<u8>().unwrap())
        .collect();

    let first_solution = simulate(80, &fishes);
    let second_solution = simulate(256, &fishes);
    println!("{}", first_solution);
    println!("{}", second_solution);
}

fn simulate(days: u32, fishes: &[u8]) -> usize {
    let mut fishes: HashMap<u8, usize> = fishes.iter().cloned().counts();
    for _ in 0..days {
        fishes = fishes
            .iter()
            .sorted()
            .rev()
            .fold(HashMap::new(), |mut fishes, (fish, amount)| {
                if *fish == 0 {
                    fishes.insert(8, *amount);
                    fishes.insert(6, fishes.get(&6).unwrap_or(&0) + *amount);
                } else {
                    fishes.insert(*fish - 1, *amount);
                }
                fishes
            });
    }
    fishes.values().sum()
}
