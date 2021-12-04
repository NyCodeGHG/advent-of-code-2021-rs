use std::{thread::sleep};

use bingo::{BingoGame};
use input::read_input;

mod bingo;
mod input;

fn main() {
    ansi_term::enable_ansi_support().unwrap();
    let game = read_input();
    let display: bool = std::env::args()
        .nth(1).map(|arg| arg == "--display")
        .unwrap_or(false);
    first_part(game, &display);
    let game = read_input();
    second_part(game);
}

fn first_part(mut game: BingoGame, display: &bool) {
    while game.draw().is_ok() {
        if *display {
            clear_screen();
            sleep(std::time::Duration::from_millis(1000));
            println!("Current Draw: {}\n", game.last_drawn.unwrap());
            game.print();
        }
        if game.finished() {
            break;
        }
    }
    let winner = game.get_winner().unwrap();
    let last_drawn = game.last_drawn.expect("No last drawn");
    let result: u32 = winner.score(last_drawn);
    println!("{}", result);
}

fn second_part(mut game: BingoGame) {
    while game.draw().is_ok() && !game.boards.iter().all(|b| b.has_won().is_some()) {}
    let last_board = game.winners.last().unwrap();
    let last_board = game.boards.get(*last_board).unwrap();
    let last_drawn = game.last_drawn.expect("No last drawn");
    let result: u32 = last_board.score(last_drawn);
    println!("{}", result);
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
