use std::fs;

fn main() {
    let numbers: Vec<i32> = fs::read_to_string("inputs/day01.txt")
        .expect("Unable to read input")
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect();

    let first = numbers
        .windows(2)
        .filter(|window| window.first() < window.last())
        .count();
    let second = numbers
        .windows(4)
        .filter(|window| window.first() < window.last())
        .count();
    println!("First Solution: {}\nSecond Solution: {}", first, second);
}
