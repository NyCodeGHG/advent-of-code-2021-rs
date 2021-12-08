use itertools::Itertools;
use std::collections::HashMap;

pub fn determine_pattern(wires: &str) -> HashMap<char, char> {
    let mut results = HashMap::new();
    let wires = wires.split(' ').collect_vec();

    let all_chars = wires.iter().flat_map(|w| w.chars()).counts();
    for x in 'a'..='g' {
        let count = all_chars.get(&x).unwrap();
        if count == &9 {
            results.insert('f', x);
            continue;
        }
        if count == &4 {
            results.insert('e', x);
            continue;
        }
        if count == &6 {
            results.insert('b', x);
            continue;
        }
    }

    find_char(&wires, &mut results, 2, 'c');
    find_char(&wires, &mut results, 3, 'a');
    find_char(&wires, &mut results, 4, 'd');
    find_char(&wires, &mut results, 7, 'g');

    results.iter().map(|(a, b)| (*b, *a)).collect()
}

fn find_char(
    wires: &[&str],
    results: &mut HashMap<char, char>,
    length: usize,
    character: char,
) -> char {
    let text = wires.iter().find(|wire| wire.len() == length).unwrap();
    let c = text
        .chars()
        .find(|c| !results.values().contains(c))
        .unwrap();
    results.insert(character, c);
    c
}

pub fn convert_number(pattern: &HashMap<char, char>, number: &str) -> u32 {
    let number: String = number
        .chars()
        .map(|c| pattern.get(&c).unwrap())
        .sorted()
        .collect();
    match number.as_str() {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => panic!("Invalid input"),
    }
}
