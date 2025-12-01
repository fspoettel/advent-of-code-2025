advent_of_code::solution!(1);

fn parse_line(l: &str) -> i64 {
    let parts = l.split_at(1);

    let coeff = match parts.0 {
        "L" => -1,
        _ => 1,
    };

    let steps: i64 = parts.1.parse().unwrap();
    coeff * steps
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut clicks = 0;
    let mut val = 50;

    for line in input.lines().filter(|line| !line.is_empty()) {
        let diff = parse_line(line);
        val = (val + diff).rem_euclid(100);
        if val == 0 {
            clicks += 1;
        }
    }

    Some(clicks)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut clicks = 0;
    let mut val: i64 = 50;

    for line in input.lines().filter(|line| !line.is_empty()) {
        let diff = parse_line(line);
        let is_left = diff.is_negative();

        let target: i64 = if is_left {
            if val == 0 { 100 } else { 0 }
        } else {
            100
        };

        let steps = diff.abs();
        let steps_to_wrap = (target - val.abs()).abs();

        if steps >= steps_to_wrap {
            let leftover = (diff).abs() - steps_to_wrap.abs();
            clicks += 1 + (leftover / 100);
        }

        val = (val + diff).rem_euclid(100);
    }

    Some(clicks)
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
        assert_eq!(result, Some(6));
    }
}
