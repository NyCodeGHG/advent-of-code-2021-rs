use std::fs;
use std::fmt::Debug;

fn main() {
    let mut fishes: Vec<Lanternfish> = fs::read_to_string("inputs/day06.txt")
        .expect("Unable to read input")
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse::<u8>().unwrap())
        .map(|value| Lanternfish { value })
        .collect();

    for _ in 0..80 {
        let mut new_fishes = vec![];
        for fish in fishes.iter_mut() {
            let result = fish.next_day();
            if result.is_some() {
                new_fishes.push(result.unwrap());
            }
        }
        fishes.append(&mut new_fishes);
    }
    println!("{}", fishes.len())
}

struct Lanternfish {
    value: u8,
}

impl Debug for Lanternfish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value.to_string())
    }
}

impl Lanternfish {
    fn next_day(&mut self) -> Option<Lanternfish> {
        if self.value == 0 {
            self.value = 6;
            return Some(Lanternfish {
                value: 8
            });
        }
        self.value = self.value - 1;
        None
    }
}
