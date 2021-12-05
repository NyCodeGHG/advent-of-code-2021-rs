use std::{fs};

use day_05::Line;
use itertools::Itertools;

fn main() {
    let overlapping_lines = fs::read_to_string("inputs/day05.txt")
        .expect("Unable to read input")
        .lines()
        .filter_map(|line| Line::from_string(line).ok())
        .filter_map(|area| area.get_line_points())
        .flatten()
        .counts()
        .iter()
        .filter(|(_, count)| **count >= 2)
        .count();
    println!("{}", overlapping_lines);
}
