use advent_of_code::{execute, parse_machine};
use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let (_, mut machine) = parse_machine(input).unwrap();

    machine.inputs.push_back(1);
    execute(&mut machine);

    println!("{}", machine.outputs.iter().join(","));

    Some(machine.outputs.pop_front().unwrap())
}

pub fn part_two(_input: &str) -> Option<i64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // technically only one of the series but close enough
        assert_eq!(result, Some(109));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(1219070632396864));
    }

    #[test]
    fn test_part_one_c() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(1125899906842624));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
