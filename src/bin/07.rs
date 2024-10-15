use std::cmp::max;

use advent_of_code::{execute, parse_machine};

use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<i32> {
    let (_, machine) = parse_machine(input).unwrap();

    let mut maximum = 0;
    let combinations = (0..=4).permutations(5).collect::<Vec<_>>();
    for combination in combinations {
        let mut io = 0;
        for phase_setting in combination.iter() {
            let mut amp = machine.clone();
            amp.inputs.push_back(*phase_setting);
            amp.inputs.push_back(io);

            execute(&mut amp);

            io = amp.outputs.pop_front().unwrap();
        }
        // input to the thruster
        maximum = max(maximum, io);
    }

    Some(maximum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43210));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(54321));
    }

    #[test]
    fn test_part_one_c() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(65210));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
