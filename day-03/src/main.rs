use std::fs;

fn main() {
    let (numbers, length) = read_input();
    let gamma = construct_number(length, &numbers, |zeros, ones| {
        if zeros.len() > ones.len() {
            0
        } else {
            1
        }
    });
    dbg!(gamma);
    let epsilon = construct_number(length, &numbers, |zeros, ones| {
        if zeros.len() > ones.len() {
            1
        } else {
            0
        }
    });
    dbg!(epsilon);
    dbg!(gamma * epsilon);
}

fn construct_number<F>(length: usize, numbers: &[u32], value: F) -> usize
where
    F: Fn(Vec<&u32>, Vec<&u32>) -> usize,
{
    (0..length)
        .rev()
        .map(|length| get_common_bit(numbers, length, &value))
        .fold(0, |acc, value| acc << 1 | value)
}

fn get_common_bit<F>(numbers: &[u32], length: usize, value: F) -> usize
where
    F: Fn(Vec<&u32>, Vec<&u32>) -> usize,
{
    let (zeros, ones): (Vec<&u32>, Vec<&u32>) = numbers
        .iter()
        .partition(|number| *number >> length & 0b1 == 0);
    value(zeros, ones)
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
