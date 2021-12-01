use std::fs;

fn main() {
    let numbers = read_input();
    let count = count_increasements(&numbers);
    println!("{}", count);

    let count = count_increasements(
        &numbers
            .windows(3)
            .map(|window| window[0] + window[1] + window[2])
            .collect::<Vec<i32>>(),
    );
    println!("{}", count);
}

fn count_increasements(numbers: &[i32]) -> usize {
    numbers
        .windows(2)
        .map(|window| window[0] < window[1])
        .filter(|value| *value)
        .count()
}

fn read_input() -> Vec<i32> {
    fs::read_to_string("inputs/day01.txt")
        .expect("Unable to read input")
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect()
}
