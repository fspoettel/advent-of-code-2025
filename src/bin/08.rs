use std::{collections::HashMap, hash::Hash};

use itertools::Itertools;

advent_of_code::solution!(8);

type Edge = (Point, Point);

#[derive(Clone, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn euclidean_distance(&self, other: &Point) -> f32 {
        [self.x - other.x, self.y - other.y, self.z - other.z]
            .into_iter()
            .map(|x| x.pow(2) as f32)
            .sum::<f32>()
            .sqrt()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_inner(input, 1000)
}

fn part_one_inner(input: &str, max: usize) -> Option<usize> {
    let (edges, _) = parse(input);
    connect(&edges, |i, _| i == max - 1)
        .map(|(counts, _)| counts.values().sorted().rev().take(3).product())
}

pub fn part_two(input: &str) -> Option<isize> {
    let (edges, point_count) = parse(input);

    connect(&edges, |_, counts| {
        counts.values().any(|v| *v == point_count)
    })
    .map(|(_, edge)| edge.0.x * edge.1.x)
}

fn parse(input: &str) -> (Vec<Edge>, usize) {
    let points = input.lines().filter_map(|l| {
        l.split(",")
            .filter_map(|s| s.parse().ok())
            .collect_tuple()
            .map(|(x, y, z)| Point { x, y, z })
    });

    let len = points.clone().count();

    let edges = points
        .tuple_combinations()
        .sorted_by(|a: &Edge, b: &Edge| {
            let a_dist = a.0.euclidean_distance(&a.1);
            let b_dist = b.0.euclidean_distance(&b.1);
            a_dist.total_cmp(&b_dist)
        })
        .collect_vec();

    (edges, len)
}

// BITCONNEEEEEEECT
fn connect(
    edges: &[(Point, Point)],
    stop_condition: impl Fn(usize, &HashMap<usize, usize>) -> bool,
) -> Option<(HashMap<usize, usize>, Edge)> {
    let mut id_counter = 1;
    let mut circuits: HashMap<Point, usize> = HashMap::new();
    let mut counts: HashMap<usize, usize> = HashMap::new();

    for (i, (a, b)) in edges.iter().enumerate() {
        let curr_a = circuits.get(a).cloned();
        let curr_b = circuits.get(b).cloned();

        match (curr_a, curr_b) {
            (Some(a_id), Some(b_id)) => {
                if a_id != b_id {
                    let mut changed = 0;
                    circuits
                        .iter_mut()
                        .filter(|entry| *entry.1 == b_id)
                        .for_each(|entry| {
                            if *entry.1 != a_id {
                                *entry.1 = a_id;
                                changed += 1;
                            }
                        });
                    counts.remove(&b_id);
                    *counts.entry(a_id).or_default() += changed;
                }
            }
            (Some(a_id), None) => {
                circuits.insert(b.clone(), a_id);
                *counts.entry(a_id).or_default() += 1;
            }
            (None, Some(b_id)) => {
                circuits.insert(a.clone(), b_id);
                *counts.entry(b_id).or_default() += 1;
            }
            (None, None) => {
                circuits.extend([(a.clone(), id_counter), (b.clone(), id_counter)]);
                counts.insert(id_counter, 2);
                id_counter += 1;
            }
        }

        if stop_condition(i, &counts) {
            return Some((counts.clone(), (a.clone(), b.clone())));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_inner(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
