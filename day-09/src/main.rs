use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day09.txt").expect("Unable to read input");
    let length = input.lines().next().unwrap().len();
    let numbers: Vec<u32> = input
        .lines()
        .flat_map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect();
    let sum: u32 = numbers
        .iter()
        .enumerate()
        .filter(|(i, number)| {
            get_neighbours(&numbers, *i, length)
                .iter()
                .all(|n| n > *number)
        })
        .map(|(_, n)| n + 1)
        .sum();
    println!("{}", sum);
}

fn get_neighbours(numbers: &[u32], index: usize, length: usize) -> Vec<u32> {
    let row_index = index % length;
    let left = if row_index != 0 {
        numbers.get(index - 1)
    } else {
        None
    };
    let top = if index >= length {
        numbers.get(index - length)
    } else {
        None
    };
    let right = if row_index != length - 1 {
        numbers.get(index + 1)
    } else {
        None
    };
    let bottom = numbers.get(index + length);
    left.iter()
        .chain(top.iter())
        .chain(right.iter())
        .chain(bottom.iter())
        .map(|f| **f)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn neighbours() {
        let test = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        // 1 2 3
        // 4 5 6
        // 7 8 9
        assert_eq!(get_neighbours(&test, 0, 3), vec![2, 4]);
        assert_eq!(get_neighbours(&test, 1, 3), vec![1, 3, 5]);
        assert_eq!(get_neighbours(&test, 2, 3), vec![2, 6]);
        assert_eq!(get_neighbours(&test, 3, 3), vec![1, 5, 7]);
        assert_eq!(get_neighbours(&test, 4, 3), vec![4, 2, 6, 8]);
        assert_eq!(get_neighbours(&test, 5, 3), vec![5, 3, 9]);
        assert_eq!(get_neighbours(&test, 6, 3), vec![4, 8]);
        assert_eq!(get_neighbours(&test, 7, 3), vec![7, 5, 9]);
        assert_eq!(get_neighbours(&test, 8, 3), vec![8, 6]);
    }
}
