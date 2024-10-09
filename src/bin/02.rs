use advent_of_code::parse_usize;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};

use itertools::Itertools;

advent_of_code::solution!(2);

#[derive(Debug)]
struct IntcodeMachine {
    program: Vec<usize>,
    instruction_pointer: usize,
}

fn parser(i: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(","), parse_usize)(i)
}

fn execute(machine: &mut IntcodeMachine) -> bool {
    let current_instruction = machine.program.get(machine.instruction_pointer);
    if let Some(opcode) = current_instruction {
        match opcode {
            1 => {
                let addition = machine
                    .program
                    .get(machine.instruction_pointer..=machine.instruction_pointer + 3)
                    .expect("opcode read out of bounds")
                    .to_owned();
                machine.program[addition[3]] =
                    machine.program[addition[1]] + machine.program[addition[2]];
            }
            2 => {
                let multiplication = machine
                    .program
                    .get(machine.instruction_pointer..=machine.instruction_pointer + 3)
                    .expect("opcode read out of bounds")
                    .to_owned();
                machine.program[multiplication[3]] =
                    machine.program[multiplication[1]] * machine.program[multiplication[2]];
            }
            99 => {
                return false;
            }
            _ => panic!("unknown opcode: {}", opcode),
        }
    } else {
        panic!("instruction pointer out of bounds")
    }
    machine.instruction_pointer += 4;
    // still active
    true
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, program) = parser(input).unwrap();
    let mut machine = IntcodeMachine {
        program,
        instruction_pointer: 0,
    };

    machine.program[1] = 12;
    machine.program[2] = 2;

    loop {
        let active = execute(&mut machine);
        if !active {
            break
        }
    }

    Some(machine.program[0])
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // the first test involved replacing values which breaks the tests if used directly
    fn test_part_one() {
        let (_, program) = parser(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        let mut machine = IntcodeMachine {
            program,
            instruction_pointer: 0,
        };

        loop {
            let active = execute(&mut machine);
            if !active {
                break
            }
        }

        assert_eq!(Some(machine.program[0]), Some(3500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}