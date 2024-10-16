use advent_of_code::{execute, parse_machine};

use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i64> {
    let (_, mut machine) = parse_machine(input).unwrap();

    machine.program[1] = 12;
    machine.program[2] = 2;

    execute(&mut machine);

    Some(machine.program[0])
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, machine) = parse_machine(input).unwrap();

    // can't run off the end of the program
    let cap = machine.program.len() - 1;
    for (noun, verb) in (0..=cap).cartesian_product(0..=cap) {
        let mut trial = machine.clone();
        trial.program[1] = noun.try_into().unwrap();
        trial.program[2] = verb.try_into().unwrap();

        execute(&mut trial);

        if trial.program[0] == 19690720 {
            return Some(100 * noun + verb);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // the first test involved replacing values which breaks the tests if used directly
    fn test_part_one() {
        let (_, mut machine) =
            parse_machine(&advent_of_code::template::read_file("examples", DAY)).unwrap();

        execute(&mut machine);

        assert_eq!(Some(machine.program[0]), Some(3500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
