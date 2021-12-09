use itertools::{iproduct, Itertools};
use pathfinding::prelude::{bfs_reach, Matrix};
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day09.txt").expect("Unable to read input");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn lows(m: &Matrix<u8>) -> impl Iterator<Item = (usize, usize)> + '_ {
    iproduct!(0..m.rows, 0..m.columns).filter(|&k| m.neighbours(k, false).all(|n| m[n] > m[k]))
}

fn part1(input: &str) -> u32 {
    let m: Matrix<u8> = input.lines().map(|c| c.bytes()).collect();
    lows(&m).map(|k| (m[k] - 48) as u32 + 1).sum()
}

fn part2(input: &str) -> usize {
    let m = input.lines().map(|c| c.bytes()).collect();
    lows(&m)
        .map(|n| {
            bfs_reach(n, |&n| {
                m.neighbours(n, false)
                    .filter(|&k| m[k] != 57 && m[k] > m[n])
                    .collect_vec()
            })
            .count()
        })
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(3)
        .product()
}
