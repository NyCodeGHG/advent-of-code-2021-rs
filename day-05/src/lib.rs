use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cmp::{max, min},
    ops::Range,
};

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
    pub fn get_line_points(&self, diagonal_lines: bool) -> Option<Vec<Point>> {
        let (x1, y1) = self.a;
        let (x2, y2) = self.b;
        if x1 == x2 {
            Some(
                range(&Range { start: y1, end: y2 })
                    .iter()
                    .map(|y| (x1, *y))
                    .collect(),
            )
        } else if y1 == y2 {
            Some(
                range(&Range { start: x1, end: x2 })
                    .iter()
                    .map(|x| (*x, y1))
                    .collect(),
            )
        } else if diagonal_lines {
            Some(
                range(&Range { start: x1, end: x2 })
                    .iter()
                    .zip(range(&Range { start: y1, end: y2 }).iter())
                    .map(|(x, y)| (*x, *y))
                    .collect(),
            )
        } else {
            None
        }
    }
}

fn range(range: &Range<u32>) -> Vec<u32> {
    let a = min(range.start, range.end);
    let b = max(range.start, range.end);
    let mut result = (a..=b).collect::<Vec<u32>>();
    if range.start > range.end {
        result.reverse();
    }
    result
}
