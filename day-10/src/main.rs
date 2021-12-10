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
    let chars: HashMap<char, char> = collection! { ']' => '[', ')' => '(', '}' => '{', '>' => '<' };
    let points: u32 = fs::read_to_string("inputs/day10.txt")
        .expect("Unable to read input")
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
        .sum();
    println!("{}", points);
}
