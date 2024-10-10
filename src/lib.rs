use nom::{
    bytes::complete::tag,
    character::complete::{digit1, i32},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

use std::str::FromStr;
pub mod template;

// Use this file to add helper functions and additional modules.

pub fn parse_usize(i: &str) -> IResult<&str, usize> {
    map_res(digit1, usize::from_str)(i)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct CoordinateSigned {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct IntcodeMachine {
    pub program: Vec<i32>,
    pub instruction_pointer: usize,
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    Position,
    Immediate,
}

fn mode_from_digit(c: &char) -> Mode {
    match c {
        '0' => Mode::Position,
        '1' => Mode::Immediate,
        _ => panic!("unexpected mode {}", c),
    }
}

pub fn parse_machine(i: &str) -> IResult<&str, IntcodeMachine> {
    let (i, program) = separated_list1(tag(","), i32)(i)?;

    Ok((
        i,
        IntcodeMachine {
            program,
            instruction_pointer: 0,
        },
    ))
}

fn step(machine: &mut IntcodeMachine) -> bool {
    macro_rules! value {
        ($n:expr, $mode:expr) => {{
            if matches!($mode, Mode::Position) {
                *position!($n)
            } else if matches!($mode, Mode::Immediate) {
                $n
            } else {
                unimplemented!()
            }
        }};
    }

    macro_rules! position {
        ($n:expr) => {
            machine
                .program
                .get_mut($n as usize)
                .unwrap_or_else(|| panic!("value {} invalid index", $n))
        };
    }
    macro_rules! command {
        ($r:expr) => {{
            let result = machine
                .program
                .get(machine.instruction_pointer..machine.instruction_pointer + $r)
                .expect("opcode read out of bounds")
                .to_owned();
            machine.instruction_pointer += $r;
            result
        }};
    }

    let current_instruction = machine.program.get(machine.instruction_pointer);
    if let Some(instruction) = current_instruction {
        let opcode = instruction % 100;
        let digits = format!("{:0>5}", instruction.to_string())
            .chars()
            .collect::<Vec<_>>();
        let mode_1 = mode_from_digit(&digits[2]);
        let mode_2 = mode_from_digit(&digits[1]);
        let _mode_3 = mode_from_digit(&digits[0]);

        match opcode {
            1 => {
                let addition = command!(4);
                *position!(addition[3]) = value!(addition[1], mode_1) + value!(addition[2], mode_2);
            }
            2 => {
                let multiplication = command!(4);
                *position!(multiplication[3]) =
                    value!(multiplication[1], mode_1) * value!(multiplication[2], mode_2);
            }
            3 => {
                let save = command!(2);
                // TODO: read from user
                let input = 1;
                *position!(save[1]) = input;
            }
            4 => {
                let output = command!(2);
                println!("Output: {}", value!(output[1], mode_1));
            }
            99 => {
                // command!(1);
                return false;
            }
            _ => panic!("unknown opcode: {}", opcode),
        }
    } else {
        panic!("instruction pointer out of bounds")
    }
    // still active
    true
}

pub fn execute(machine: &mut IntcodeMachine) {
    loop {
        let active = step(machine);
        if !active {
            break;
        }
    }
}
