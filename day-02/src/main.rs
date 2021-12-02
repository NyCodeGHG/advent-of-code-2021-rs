use std::fs;

fn main() {
    let (depth, pos) = fs::read_to_string("inputs/day02.txt")
        .expect("Unable to read input")
        .lines()
        .filter_map(|line| Action::from_string(line))
        .fold((0, 0), |(depth, pos), action| match action {
            Action::Forward(n) => (depth, pos + n),
            Action::Down(n) => (depth + n, pos),
            Action::Up(n) => (depth - n, pos),
        });
    println!("Depth * Horizontal Position: {}", depth * pos);
}

enum Action {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Action {
    pub fn from_string(string: &str) -> Option<Self> {
        let split: Vec<&str> = string.split(' ').collect();
        let number: i32 = match split.last().and_then(|s| s.parse::<i32>().ok()) {
            Some(n) => n,
            None => return None,
        };
        split.first().and_then(|action| match *action {
            "forward" => Some(Self::Forward(number)),
            "down" => Some(Self::Down(number)),
            "up" => Some(Self::Up(number)),
            _ => None,
        })
    }
}
