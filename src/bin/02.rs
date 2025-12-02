use advent_of_code::count_digits;

advent_of_code::solution!(2);

fn parse(input: &str) -> Option<impl Iterator<Item = (usize, usize)>> {
    input
        .lines()
        .next()
        .map(|l| l.split(",").filter_map(parse_range))
}

fn parse_range(s: &str) -> Option<(usize, usize)> {
    let (start_s, end_s) = s.split_once("-")?;
    start_s.parse().ok().zip(end_s.parse().ok())
}

pub fn part_one(input: &str) -> Option<usize> {
    parse(input).map(|ranges| {
        ranges
            .flat_map(|(start, end)| {
                (start..=end)
                    .filter(|id| id_valid(*id, 2))
                    .collect::<Vec<usize>>()
            })
            .sum()
    })
}

pub fn part_two(input: &str) -> Option<usize> {
    parse(input).map(|ranges| {
        ranges
            .flat_map(|(start, end)| {
                (start..=end)
                    .filter(|id| {
                        let len = count_digits(*id);
                        (2..=len).into_iter().any(|n| id_valid(*id, n))
                    })
                    .collect::<Vec<usize>>()
            })
            .sum()
    })
}

fn id_valid(id: usize, parts: usize) -> bool {
    let len = count_digits(id);

    let chunk_size = len.div_ceil(parts);

    if !len.is_multiple_of(chunk_size) {
        return false;
    }

    let id_s = id.to_string();
    let mut chunks = id_s.as_bytes().chunks(chunk_size).peekable();

    let first_chunk = chunks.peek().cloned().unwrap();

    chunks.all(|chunk| chunk.iter().enumerate().all(|(i, c)| first_chunk[i] == *c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
