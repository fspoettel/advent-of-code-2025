use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

pub fn part_one(input: &str) -> Option<isize> {
    input
        .lines()
        .filter_map(|l| {
            l.split_once(",").and_then(|(x, y)| {
                Some(Point {
                    x: x.parse().ok()?,
                    y: y.parse().ok()?,
                })
            })
        })
        .tuple_combinations()
        .map(|(a, b)| {
            let x = (a.x - b.x).abs() + 1;
            let y = (a.y - b.y).abs() + 1;
            x * y
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
