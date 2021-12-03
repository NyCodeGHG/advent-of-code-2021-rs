use std::{collections::HashMap, fs, hash::Hash};

fn main() -> Result<(), String> {
    let numbers: Vec<Vec<u32>> = fs::read_to_string("inputs/day03.txt")
        .expect("Unable to read input")
        .lines()
        .map(|line| line.chars().filter_map(|char| char.to_digit(10)).collect())
        .collect();

    let mut gamma_bits: Vec<u32> = Vec::new();
    let mut epsilon_bits: Vec<u32> = Vec::new();

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
        gamma_bits.push(*most);
        epsilon_bits.push(*least);
    }

    let gamma = bits_to_int(&gamma_bits);
    let epsilon = bits_to_int(&epsilon_bits);
    let power = gamma * epsilon;
    println!("Power: {}", power);

    let oxygen = find_oxygen_generator_rating(&numbers, 0).ok_or("Cannot find oxygen")?;
    let co2 = find_co2_scrubber_rating(&numbers, 0).ok_or("Cannot find co2")?;
    println!("Life Support Rating: {}", oxygen * co2);
    Ok(())
}

fn find_oxygen_generator_rating(numbers: &[Vec<u32>], position: usize) -> Option<i32> {
    if numbers.len() == 1 {
        return numbers.first().map(|value| bits_to_int(value));
    }
    if position >= numbers.first().unwrap().len() {
        return None;
    }
    let pos_numbers: Vec<&u32> = numbers.iter().filter_map(|v| v.get(position)).collect();
    let most = get_most_frequent_value(&pos_numbers);
    let target_value = most.unwrap_or(&1);
    let numbers: Vec<Vec<u32>> = numbers
        .iter()
        .filter(|number| number.get(position).unwrap() == target_value)
        .cloned()
        .collect();
    find_oxygen_generator_rating(&numbers, position + 1)
}

fn find_co2_scrubber_rating(numbers: &[Vec<u32>], position: usize) -> Option<i32> {
    if numbers.len() == 1 {
        return numbers.first().map(|value| bits_to_int(value));
    }
    if position >= numbers.first().unwrap().len() {
        return None;
    }
    let pos_numbers: Vec<&u32> = numbers.iter().filter_map(|v| v.get(position)).collect();
    let least = get_least_frequent_value(&pos_numbers);
    let target_value = least.unwrap_or(&0);
    let numbers: Vec<Vec<u32>> = numbers
        .iter()
        .filter(|number| number.get(position).unwrap() == target_value)
        .cloned()
        .collect();
    find_co2_scrubber_rating(&numbers, position + 1)
}

fn get_most_frequent_value<T: Ord + Hash + Copy>(values: &[T]) -> Option<T> {
    let mut counter: HashMap<T, u32> = HashMap::new();
    for value in values {
        counter.insert(*value, counter.get(value).unwrap_or(&0) + 1);
    }
    let highest = counter.iter().map(|(_, i)| i).max()?;
    let most_values: Vec<&T> = counter
        .iter()
        .filter(|(_, count)| highest == *count)
        .map(|value| value.0)
        .collect();
    if most_values.len() != 1 {
        return None;
    }
    Some(**most_values.get(0).unwrap())
}

fn get_least_frequent_value<T: Ord + Hash + Copy>(values: &[T]) -> Option<T> {
    let mut counter: HashMap<T, u32> = HashMap::new();
    for value in values {
        counter.insert(*value, counter.get(value).unwrap_or(&0) + 1);
    }
    let lowest = counter.iter().map(|(_, i)| i).min()?;
    let least_values: Vec<&T> = counter
        .iter()
        .filter(|(_, count)| lowest == *count)
        .map(|value| value.0)
        .collect();
    if least_values.len() != 1 {
        return None;
    }
    Some(**least_values.get(0).unwrap())
}

fn bits_to_int(bits: &[u32]) -> i32 {
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
