use std::thread::sleep;

use input::read_input;

mod bingo;
mod input;

fn main() {
    ansi_term::enable_ansi_support().unwrap();
    let mut game = read_input();
    let display: bool = std::env::args()
        .nth(1)
        .and_then(|arg| Some(arg == "--display"))
        .unwrap_or(false);
    while game.draw().is_ok() {
        if display {
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
    let result: u64 = winner
        .numbers
        .iter()
        .filter(|n| !n.drawn)
        .map(|n| n.number as u64)
        .sum::<u64>()
        * last_drawn as u64;
    println!("{}", result);
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
