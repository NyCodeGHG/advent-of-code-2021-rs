use std::fs;

fn main() {
    let actions: Vec<Action> = fs::read_to_string("inputs/day02.txt")
        .expect("Unable to read input")
        .lines()
        .filter_map(|line| Action::from_string(line))
        .collect();
    let (depth, pos) = actions
        .iter()
        .fold((0, 0), |(depth, pos), action| match action {
            Action::Forward(n) => (depth, pos + n),
            Action::Down(n) => (depth + n, pos),
            Action::Up(n) => (depth - n, pos),
        });
    println!("Depth * Horizontal Position: {}", depth * pos);

    let (depth, pos, _) =
        actions
            .iter()
            .fold((0, 0, 0), |(depth, pos, aim), action| match action {
                Action::Forward(n) => (depth + aim * n, pos + n, aim),
                Action::Down(n) => (depth, pos, aim + n),
                Action::Up(n) => (depth, pos, aim - n),
            });
    println!("Depth * Horizontal Position (Aim Method): {}", depth * pos);
}

enum Action {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Action {
    pub fn from_string(string: &str) -> Option<Self> {
        let split: Vec<&str> = string.split(' ').collect();
        let number: i32 = split.last()?.parse::<i32>().ok()?;
        match *split.first()? {
            "forward" => Some(Self::Forward(number)),
            "down" => Some(Self::Down(number)),
            "up" => Some(Self::Up(number)),
            _ => None,
        }
    }
}
