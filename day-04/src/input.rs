use std::fs;

use crate::bingo::{BingoBoard, BingoGame, Number};

pub fn read_input() -> BingoGame {
    let content: Vec<String> = fs::read_to_string("inputs/day04.txt")
        .expect("Unable to read input")
        .lines()
        .map(|line| line.to_string())
        .collect();

    let mut draw_order = content.get(0)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    draw_order.reverse();

    let boards = content
        .iter()
        .skip(1)
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .flatten()
        .collect::<Vec<u32>>()
        .chunks(25)
        .map(|chunk| BingoBoard {
            numbers: chunk
                .iter()
                .map(|n| Number {
                    number: *n,
                    drawn: false,
                })
                .collect(),
        })
        .collect();

    BingoGame {
        draw_order,
        boards,
        last_drawn: None,
        winners: vec![],
    }
}
