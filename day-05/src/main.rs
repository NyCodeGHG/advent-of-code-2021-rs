use std::fs;

use day_05::Line;
use itertools::Itertools;

fn main() {
    println!("{}", overlapping_lines(false));
    println!("{}", overlapping_lines(true));
}

fn overlapping_lines(diagonal_lines: bool) -> usize {
    fs::read_to_string("inputs/day05.txt")
        .expect("Unable to read input")
        .lines()
        .filter_map(|line| Line::from_string(line).ok())
        .filter_map(|area| area.get_line_points(diagonal_lines))
        .flatten()
        .counts()
        .iter()
        .filter(|(_, count)| **count >= 2)
        .count()
}
