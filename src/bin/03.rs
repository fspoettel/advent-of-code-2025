advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 12))
}

fn solve(input: &str, size: usize) -> u64 {
    input
        .lines()
        .map(|l| {
            let battery: Vec<u8> = l
                .as_bytes()
                .iter()
                .map(|c| c - b'0')
                .collect::<Vec<_>>();

            switch_battery(&battery, size)
        })
        .sum::<u64>()
}

fn switch_battery(battery: &[u8], size: usize) -> u64 {
    let mut joltage: Vec<u8> = vec![0; size];
    let len = battery.len();

    for (i, x) in battery.iter().enumerate() {
        let remaining = size.saturating_sub(len - i);

        for j in remaining..size {
            let curr = joltage[j];

            if *x > curr {
                joltage[j] = *x;

                for item in joltage.iter_mut().take(size).skip(j + 1) {
                    *item = 0;
                }

                break;
            }
        }
    }

    joltage.into_iter().enumerate().fold(0, |acc, (i, curr)| {
        let factor = size - i - 1;
        acc + 10_u64.pow(factor as u32) * (curr as u64)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
