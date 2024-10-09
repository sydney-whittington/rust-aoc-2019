use nom::{
    character::complete::{i32, newline},
    multi::separated_list0,
    IResult,
};

advent_of_code::solution!(1);

fn parser(i: &str) -> IResult<&str, Vec<i32>> {
    separated_list0(newline, i32)(i)
}

fn calculate_fuel(w: &i32) -> i32 {
    let fuel = (*w / 3) - 2;
    if fuel > 0 {
        fuel + calculate_fuel(&fuel)
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let (_, nums) = parser(input).unwrap();
    Some(nums.iter().map(|n| n / 3 - 2).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let (_, nums) = parser(input).unwrap();
    Some(nums.iter().map(calculate_fuel).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2 + 2 + 654 + 33583));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2 + 2 + 966 + 50346));
    }
}
