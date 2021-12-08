use std::fs;
use itertools::Itertools;

fn main() {
    let result = fs::read_to_string("inputs/day08.txt")
        .expect("Unable to read input")
        .lines()
        .map(|line| line.split('|').nth(1).unwrap().trim())
        .flat_map(|line| line.split(' ').collect_vec())
        .filter(|part| matches!(part.len(), 2 | 4 | 3 | 7))
        .count();
    println!("{}", result);
}
