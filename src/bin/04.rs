use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::u32, sequence::separated_pair, IResult};

advent_of_code::solution!(4);

fn parser(i: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(u32, tag("-"), u32)(i)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, (lower, upper)) = parser(input).unwrap();
    let candidates = (lower..=upper).filter(|n| {
        let digit_pairs: Vec<(char, char)> = n.to_string().chars().tuple_windows().collect();
        digit_pairs.iter().any(|(a, b)| a == b)
            && digit_pairs
                .iter()
                .all(|(a, b)| a.to_digit(10) <= b.to_digit(10))
    });

    Some(candidates.collect::<Vec<_>>().len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, (lower, upper)) = parser(input).unwrap();
    let candidates = (lower..=upper).filter(|n| {
        let digit_pairs: Vec<(char, char)> = n.to_string().chars().tuple_windows().collect();
        digit_pairs.iter().all(|(a, b)| a.to_digit(10) <= b.to_digit(10))
            // there is a 2-length consecutive run of the same digit
            && n.to_string().chars().chunk_by(|a| *a).into_iter().any(|(_, c)| c.count() == 2)
    });

    Some(candidates.collect::<Vec<_>>().len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
