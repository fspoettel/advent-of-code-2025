use advent_of_code::{ALL_DIRECTIONS, Grid, Point};
use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    let (_, removed) = transform(&grid);
    Some(removed)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = parse(input);
    let mut total = 0;

    loop {
        let (next_grid, removed) = transform(&grid);
        if removed != 0 {
            grid = next_grid;
            total += removed;
        } else {
            break;
        }
    }

    Some(total)
}

fn parse(input: &str) -> Grid<bool> {
    Grid::from(
        input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().map(|c| c == '@').collect())
            .collect::<Vec<Vec<bool>>>(),
    )
}

fn transform(grid: &Grid<bool>) -> (Grid<bool>, u32) {
    let mut remove_count = 0;

    let mut next_grid: Vec<Vec<bool>> = grid.cells.clone();

    for (y, x) in (0..grid.rows).cartesian_product(0..grid.cols) {
        let point = Point {
            x: x as isize,
            y: y as isize,
        };

        let removed = grid.get(&point)
            && ALL_DIRECTIONS
                .iter()
                .filter(|dir| {
                    grid.neighbor(&point, dir)
                        .map(|n| grid.get(&n))
                        .unwrap_or(false)
                })
                .count()
                < 4;

        if removed {
            remove_count += 1;
            next_grid[y][x] = false;
        }
    }

    (Grid::from(next_grid), remove_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
