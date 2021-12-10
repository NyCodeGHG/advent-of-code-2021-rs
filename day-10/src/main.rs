use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$(($k, $v),)*]))
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$($v,)*]))
    }};
}

fn main() {
    let input = fs::read_to_string("inputs/day10.txt").expect("Unable to read input");
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> u32 {
    let chars: HashMap<char, char> = collection! { ']' => '[', ')' => '(', '}' => '{', '>' => '<' };
    input
        .lines()
        .map(|line| {
            let mut chunks: Vec<char> = Vec::new();
            for char in line.chars() {
                let required = match chars.get(&char) {
                    Some(c) => c,
                    None => {
                        chunks.push(char);
                        continue;
                    }
                };
                let last = match chunks.pop() {
                    Some(c) => c,
                    None => panic!("No last value"),
                };
                if *required != last {
                    return match required {
                        '(' => 3,
                        '[' => 57,
                        '{' => 1197,
                        '<' => 25137,
                        _ => 0,
                    };
                }
            }
            0
        })
        .sum()
}

fn part_two(input: &str) -> u64 {
    let chars: HashMap<char, char> = collection! { '[' => ']', '(' => ')', '{' => '}', '<' => '>' };
    let result = input
        .lines()
        .filter(|line| is_not_corrupted(line))
        .map(|line| {
            let mut chunks: Vec<char> = Vec::new();
            for char in line.chars() {
                if let Some(c) = chars.get(&char) {
                    chunks.push(*c);
                } else {
                    chunks.pop();
                }
            }
            chunks.reverse();
            let result = chunks
                .iter()
                .map(|char| match *char {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                })
                .fold(0, |acc, value| acc * 5 + value);
            result
        })
        .sorted()
        .collect_vec();
    *result.get(result.len() / 2).unwrap()
}

fn is_not_corrupted(line: &str) -> bool {
    let chars: HashMap<char, char> = collection! { ']' => '[', ')' => '(', '}' => '{', '>' => '<' };
    let mut chunks: Vec<char> = Vec::new();
    for char in line.chars() {
        let required = match chars.get(&char) {
            Some(c) => c,
            None => {
                chunks.push(char);
                continue;
            }
        };
        let last = match chunks.pop() {
            Some(c) => c,
            None => panic!("No last value"),
        };
        if *required != last {
            return false;
        }
    }
    true
}
