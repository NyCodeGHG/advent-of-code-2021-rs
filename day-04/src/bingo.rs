use ansi_term::Color::Green;
use ansi_term::Style;

pub struct BingoBoard {
    pub numbers: Vec<Number>,
}

#[derive(Clone, Copy)]
pub struct Number {
    pub number: u32,
    pub drawn: bool,
}

pub struct BingoGame {
    pub boards: Vec<BingoBoard>,
    pub draw_order: Vec<u32>,
    pub last_drawn: Option<u32>,
}

impl BingoGame {
    pub fn draw(&mut self) -> Result<u32, String> {
        let number = self.draw_order.pop().ok_or("No more numbers to draw")?;
        self.last_drawn = Some(number);
        for board in self.boards.iter_mut() {
            board
                .numbers
                .iter_mut()
                .find(|n| n.number == number)
                .map(|number| {
                    number.drawn = true;
                });
        }
        Ok(number)
    }
    pub fn print(&self) {
        for board in self.boards.iter() {
            board.print();
            println!("");
        }
    }

    pub fn finished(&self) -> bool {
       self.boards.iter().any(|board| board.has_won().is_some())
    }
    
    pub fn get_winner(&self) -> Option<&BingoBoard> {
        self.boards.iter().find(|board| board.has_won().is_some())
    }
}

impl BingoBoard {
    pub fn print(&self) {
        let win = self.has_won();
        self.numbers.chunks(5).for_each(|row| {
            for number in row.iter() {
                let formatted = if number.number < 10 {
                    format!(" {}", number.number)
                } else {
                    number.number.to_string()
                };
                let won = match &win {
                    Some(n) => n.contains(&number.number),
                    None => false,
                };
                let result = if number.drawn {
                    Style::new().bold().paint(formatted)
                } else {
                    Style::default().paint(formatted)
                };
                let final_result = if won {
                    Green.paint(result.to_string())
                } else {
                    result
                };
                print!("{} ", final_result);
            }
            println!("")
        });
    }

    pub fn has_won(&self) -> Option<Vec<u32>> {
        // Check for horizontal, vertical, and diagonal wins
        let horizontal: Vec<u32> = self
            .numbers
            .chunks(5)
            .filter(|row| row.iter().all(|n| n.drawn))
            .map(|row| row.iter().map(|n| n.number).collect::<Vec<u32>>())
            .flatten()
            .collect();

        // Check for vertical wins in the bingo board
        let mut vertical = vec![Number {
            number: 0,
            drawn: false,
        }; 25];
        transpose::transpose(&self.numbers, &mut vertical, 5, 5);
        let vertical: Vec<u32> = vertical
            .chunks(5)
            .filter(|row| row.iter().all(|n| n.drawn))
            .map(|row| row.iter().map(|n| n.number).collect::<Vec<u32>>())
            .flatten()
            .collect();

        if horizontal.len() > 0 && vertical.len() > 0 {
            Some(horizontal.into_iter().chain(vertical).collect())
        } else if horizontal.len() > 0 {
            Some(horizontal)
        } else if vertical.len() > 0 {
            Some(vertical)
        } else {
            None
        }
    }
}
