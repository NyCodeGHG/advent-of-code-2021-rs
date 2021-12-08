use day_08::{convert_number, determine_pattern};
use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day08.txt").expect("Unable to read input");
    let lines = input.lines();

    let result = lines
        .clone()
        .map(|line| line.split('|').nth(1).unwrap().trim())
        .flat_map(|line| line.split(' ').collect_vec())
        .filter(|part| matches!(part.len(), 2 | 4 | 3 | 7))
        .count();
    println!("{}", result);

    let result: u32 = lines
        .filter_map(|line| line.split('|').collect_tuple())
        .filter_map(|(wires, number)| {
            let pattern = &determine_pattern(wires);
            number
                .trim()
                .split(' ')
                .map(|n| convert_number(pattern, n).to_string())
                .collect::<String>()
                .parse::<u32>()
                .ok()
        })
        .sum();
    println!("{:?}", result);
}
