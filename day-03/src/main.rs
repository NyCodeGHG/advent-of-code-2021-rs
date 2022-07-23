use std::fs;

fn main() {
    let (numbers, length) = read_input();
    let most_common = |zeros: Vec<&u32>, ones: Vec<&u32>| {
        if zeros.len() > ones.len() {
            0
        } else {
            1
        }
    };
    let least_common = |zeros: Vec<&u32>, ones: Vec<&u32>| {
        if zeros.len() > ones.len() {
            1
        } else {
            0
        }
    };
    let gamma = filter_by_bit_horizontal(length, &numbers, &most_common);
    let epsilon = filter_by_bit_horizontal(length, &numbers, &least_common);
    println!("Power Consumption: {}", gamma * epsilon);

    let oxygen_generator_rating = filter_by_bit_vertical(&numbers, length, &most_common);
    let co2_scrubber_rating = filter_by_bit_vertical(&numbers, length, &least_common);
    println!(
        "Life Support Rating: {}",
        oxygen_generator_rating * co2_scrubber_rating
    );
}

fn filter_by_bit_horizontal<F>(length: usize, numbers: &[u32], value: F) -> u32
where
    F: Fn(Vec<&u32>, Vec<&u32>) -> u32,
{
    (0..length)
        .rev()
        .map(|length| get_common_bit(numbers, length, &value))
        .fold(0, |acc, value| acc << 1 | value)
}

fn get_common_bit<F>(numbers: &[u32], length: usize, value: F) -> u32
where
    F: Fn(Vec<&u32>, Vec<&u32>) -> u32,
{
    let (zeros, ones): (Vec<&u32>, Vec<&u32>) = numbers
        .iter()
        .partition(|number| *number >> length & 0b1 == 0);
    value(zeros, ones)
}

fn filter_by_bit_vertical<F>(numbers: &[u32], length: usize, value: F) -> u32
where
    F: Fn(Vec<&u32>, Vec<&u32>) -> u32,
{
    let value = (0..length).rev().fold(numbers.to_vec(), |acc, length| {
        if acc.len() == 1 {
            acc
        } else {
            let value = get_common_bit(&acc, length, &value);
            acc.into_iter()
                .filter(|i| *i >> length & 0b1 == value)
                .collect::<Vec<_>>()
        }
    });
    *value.get(0).unwrap()
}

fn read_input() -> (Vec<u32>, usize) {
    let content = fs::read_to_string("inputs/day03.txt").expect("Unable to read input");
    let length = content.lines().next().unwrap().len();
    let numbers = content
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();
    (numbers, length)
}
