use std::collections::{HashMap, HashSet};

use advent_of_code::{Grid, Point};

advent_of_code::solution!(7);

fn parse(input: &str) -> Option<(usize, Grid<char>)> {
    let chars: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    chars[0]
        .clone()
        .into_iter()
        .enumerate()
        .find(|c| c.1 == 'S')
        .map(|c| (c.0, Grid::from(chars)))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (start, grid) = parse(input)?;

    let mut split_count = 0;
    let mut beams: HashSet<usize> = HashSet::from([start]);

    (1..grid.rows).for_each(|y| {
        let mut next_beams = HashSet::new();
        beams.iter().for_each(|beam| {
            let p = grid.get(&Point {
                y: y as isize,
                x: *beam as isize,
            });

            if p == '^' {
                split_count += 1;
                next_beams.insert(beam + 1);
                next_beams.insert(beam - 1);
            } else {
                next_beams.insert(*beam);
            }
        });

        beams = next_beams;
    });

    Some(split_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (start, grid) = parse(input)?;

    let mut beams: HashMap<usize, usize> = HashMap::from([(start, 1)]);

    for y in 1..grid.rows {
        let mut next_beams: HashMap<usize, usize> = HashMap::new();

        beams.iter().for_each(|(beam, count)| {
            let val = grid.get(&Point {
                y: y as isize,
                x: *beam as isize,
            });

            let splits = if val == '^' {
                vec![beam - 1, beam + 1]
            } else {
                vec![*beam]
            };

            splits.iter().for_each(|val| {
                next_beams.insert(*val, next_beams.get(val).unwrap_or(&0) + count);
            });
        });

        beams = next_beams;
    }

    Some(beams.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
