use std::{cmp::{Ordering, min, max}, ops::RangeInclusive};

advent_of_code::solution!(5);

fn parse(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (ranges_s, id_s) = input.split_once("\n\n").unwrap();

    let ranges = ranges_s
        .lines()
        .filter_map(|range_s| {
            range_s.split_once("-")
                .and_then(|(start_s, end_s)| {
                    start_s
                    .parse()
                    .ok()
                    .zip(end_s.parse().ok())
                    .map(|(start, end)| RangeInclusive::new(start, end))
                })
        })
        .collect();

    let ids = id_s.lines().filter_map(|l| l.parse().ok()).collect();

    (ranges, ids)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ingredients) = parse(input);

    Some(
        ingredients
            .into_iter()
            .filter(|id| {
                ranges
                    .iter()
                    .any(|range| id >= range.start() && id <= range.end())
            })
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut intervals, _) = parse(input);

    intervals.sort_by(|a, b| {
        match a.start().cmp(b.start()) {
            Ordering::Equal => a.end().cmp(b.end()),
            order => order
        }
    });

    let mut merged: Vec<RangeInclusive<u64>> = vec![];

    for next in intervals {
        let mut new_interval = true;

        for (idx, prev) in merged.iter().enumerate() {
            if next.start() > prev.end() {
                continue;
            } else {
                new_interval = false;

                let start = min(prev.start(), next.start());
                let end = max(prev.end(), next.end());

                merged[idx] = RangeInclusive::new(*start, *end);

                break;
            }
        }

        if new_interval {
            merged.push(next);
        }
    }

    let total = merged
        .into_iter()
        .fold(0, |acc, curr| {
            acc + curr.count()
        });

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
