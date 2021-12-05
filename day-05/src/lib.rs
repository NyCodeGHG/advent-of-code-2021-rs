use std::cmp::{min, max};
use regex::Regex;
use lazy_static::lazy_static;

pub type Point = (u32, u32);

#[derive(Copy, Clone)]
pub struct Line {
    a: Point,
    b: Point,
}

impl Line {
    pub fn from_string(s: &str) -> Result<Line, &str> {
        lazy_static! {
            // https://regex101.com/r/8PAd2G/1
            static ref RE: Regex = Regex::new("(\\d+),(\\d+) -> (\\d+),(\\d+)").unwrap();
        }
        let captures = RE.captures(s).ok_or("Invalid input")?;
        let a = (
            captures[1].parse::<u32>().unwrap(),
            captures[2].parse::<u32>().unwrap(),
        );
        let b = (
            captures[3].parse::<u32>().unwrap(),
            captures[4].parse::<u32>().unwrap(),
        );
        Ok(Line { a, b })
    }
    pub fn get_line_points(&self) -> Option<Vec<Point>> {
        let (x1, y1) = self.a;
        let (x2, y2) = self.b;
        if x1 == x2 {
            let a = min(y1, y2);
            let b = max(y1, y2);
            Some((a..=b).map(|y| (x1, y)).collect())
        } else if y1 == y2 {
            let a = min(x1, x2);
            let b = max(x1, x2);
            Some((a..=b).map(|x| (x, y1)).collect())
        } else {
            None
        }
    }
}
