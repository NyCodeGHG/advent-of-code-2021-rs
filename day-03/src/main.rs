use std::{collections::HashMap, fs, hash::Hash, num::ParseIntError};

fn main() -> Result<(), ParseIntError> {
    let numbers: Vec<Vec<u32>> = fs::read_to_string("inputs/day03.txt")
        .expect("Unable to read input")
        .lines()
        .map(|line| line.chars().filter_map(|char| char.to_digit(10)).collect())
        .collect();

    let mut gamma_bits = Vec::new();
    let mut epsilon_bits = Vec::new();

    let length = numbers.first().unwrap().len();
    for i in 0..length {
        let numbers: Vec<&u32> = numbers.iter().filter_map(|v| v.get(i)).collect();
        let most = match get_most_frequent_value(&numbers) {
            Some(n) => n,
            None => continue,
        };
        let least = match get_least_frequent_value(&numbers) {
            Some(n) => n,
            None => continue,
        };
        gamma_bits.push(most);
        epsilon_bits.push(least);
    }

    let gamma = bits_to_int(&gamma_bits);
    let epsilon = bits_to_int(&epsilon_bits);
    let power = gamma * epsilon;
    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);
    println!("Power: {}", power);
    Ok(())
}

fn get_most_frequent_value<T: Ord + Hash + Copy>(values: &[T]) -> Option<T> {
    let mut counter: HashMap<T, u32> = HashMap::new();
    for value in values {
        counter.insert(*value, counter.get(value).unwrap_or(&0) + 1);
    }
    let highest = counter.iter().map(|(_, i)| i).max()?;
    Some(*counter.iter().find(|(_, count)| highest == *count)?.0)
}

fn get_least_frequent_value<T: Ord + Hash + Copy>(values: &[T]) -> Option<T> {
    let mut counter: HashMap<T, u32> = HashMap::new();
    for value in values {
        counter.insert(*value, counter.get(value).unwrap_or(&0) + 1);
    }
    let highest = counter.iter().map(|(_, i)| i).min()?;
    Some(*counter.iter().find(|(_, count)| highest == *count)?.0)
}

fn bits_to_int(bits: &[&u32]) -> i32 {
    i32::from_str_radix(
        &bits
            .iter()
            .map(|bit| bit.to_string())
            .collect::<Vec<String>>()
            .join(""),
        2,
    )
    .unwrap_or(0)
}
