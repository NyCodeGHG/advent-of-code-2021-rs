use std::{fs, num::ParseIntError};

fn main() {
    let actions: Vec<Action> = fs::read_to_string("inputs/day02.txt")
        .expect("Unable to read input")
        .lines()
        .filter_map(|line| Action::try_from(line).ok())
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

enum ActionError {
    ParseIntError(ParseIntError),
    NoValueError,
    InvalidAction,
}

impl From<ParseIntError> for ActionError {
    fn from(error: ParseIntError) -> Self {
        ActionError::ParseIntError(error)
    }
}

impl TryFrom<&str> for Action {
    type Error = ActionError;
    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = string.split(' ').collect();
        let number: i32 = split.last().ok_or(ActionError::NoValueError)?.parse()?;
        match *split.first().ok_or(ActionError::NoValueError)? {
            "forward" => Ok(Self::Forward(number)),
            "down" => Ok(Self::Down(number)),
            "up" => Ok(Self::Up(number)),
            _ => Err(ActionError::InvalidAction),
        }
    }
}
