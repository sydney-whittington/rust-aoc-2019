use nom::{character::complete::{newline, u32}, multi::separated_list0, IResult};

advent_of_code::solution!(1);

fn parser(i: &str) -> IResult<&str, Vec<u32>> {
    separated_list0(newline, u32)(i)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, nums) = parser(input).unwrap();
    Some(nums.iter().map(|n| n/3 - 2).sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2+2+654+33583));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
