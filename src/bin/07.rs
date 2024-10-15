use std::{cmp::max, collections::VecDeque, iter};

use advent_of_code::{execute, parse_machine, IntcodeMachine, State};

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

pub fn part_two(input: &str) -> Option<i32> {
    let (_, machine) = parse_machine(input).unwrap();

    let mut maximum = 0;
    // let combinations = (5..=9).permutations(5).collect::<Vec<_>>();
    let combinations = vec![vec![9, 8, 7, 6, 5]];
    for combination in combinations {
        let mut io: VecDeque<i32> = VecDeque::from([0]);
        let mut machines: VecDeque<IntcodeMachine> =
            iter::repeat(machine.clone()).take(5).collect();

        // initialize each with the phase settings
        for phase_setting in combination.iter() {
            let mut amp = machines.pop_front().unwrap();
            amp.inputs.push_back(*phase_setting);
            amp.inputs.extend(io.drain(..));

            execute(&mut amp);

            io.extend(amp.outputs.drain(..));
            machines.push_back(amp);
        }
        // and then just let it run
        loop {
            let mut done = advent_of_code::State::Active;
            for _ in 1..=5 {
                let mut amp = machines.pop_front().unwrap();
                amp.inputs.extend(io.drain(..));

                done = execute(&mut amp);
                io.extend(amp.outputs.drain(..));
                machines.push_back(amp);
            }
            if matches!(done, State::Terminated) {
                break;
            }
        }
        dbg!(&io);
        // input to the thruster
        maximum = max(maximum, io.pop_front().unwrap());
    }

    Some(maximum)
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
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(54321));
    }

    #[test]
    fn test_part_one_c() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(65210));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(139629729));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(18216));
    }
}
