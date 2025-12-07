advent_of_code::solution!(6);

fn parse(input: &str) -> Vec<Vec<String>> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let mut eqs: Vec<Vec<String>> = vec![];
    let mut curr = 0;

    let w = grid[0].len();
    let h = grid.len();

    (0..w).for_each(|x| {
        let column = (0..h).map(|y| grid[y][x]);

        if column.clone().any(|x| x != ' ') {
            if curr >= eqs.len() {
                eqs.push(vec![String::new(); h]);
            }

            let current_eq = &mut eqs[curr];
            for (y, c) in column.enumerate() {
                current_eq[y].push(c);
            }
        } else {
            curr += 1;
        }
    });

    eqs
}

fn eval<'a, 'b>(op: &'a str, values: impl Iterator<Item = &'b String>) -> Option<u64> {
    let values = values.filter_map(|v| v.trim().parse::<u64>().ok());
    match op.trim() {
        "*" => Some(values.product()),
        "+" => Some(values.sum()),
        _ => unreachable!()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .iter()
            .filter_map(|eq| {
                let mut eq = eq.iter().rev();
                let op = eq.next()?;
                eval(op, eq)
            })
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse(input)
        .iter()
        .filter_map(|eq| {
            let mut eq = eq.iter().rev();
            let op = eq.next()?;

            let mut values = vec![String::new(); op.len()];

            eq.rev().for_each(|s| {
                s.chars().enumerate().for_each(|(i, c)| {
                    values[i].push(c);
                });
            });

            eval(op, values.iter())
        })
        .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
