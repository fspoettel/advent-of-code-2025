advent_of_code::solution!(1);

fn parse_line(l: &str) -> Option<i32> {
    let parts = l.split_at(1);
    let coeff = if parts.0 == "L" { -1 } else { 1 };
    parts.1.parse::<i32>().map(|val| val * coeff).ok()
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut clicks = 0;
    let mut val = 50;

    input.lines().filter_map(parse_line).for_each(|diff| {
        val = (val + diff).rem_euclid(100);
        if val == 0 {
            clicks += 1;
        }
    });

    Some(clicks)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut clicks = 0;
    let mut val: i32 = 50;

    input.lines().filter_map(parse_line).for_each(|diff| {
        let target: i32 = if diff.is_negative() {
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
    });

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
