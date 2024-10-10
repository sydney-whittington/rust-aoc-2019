use advent_of_code::{execute, parse_machine};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut machine) = parse_machine(input).unwrap();

    execute(&mut machine);
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // no tests
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
        assert!(true);
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
        assert!(true);
    }
}
