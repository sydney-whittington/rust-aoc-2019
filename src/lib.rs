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
    macro_rules! position {
        ($n:expr) => {
            machine
                .program
                .get_mut($n as usize)
                .unwrap_or_else(|| panic!("value {} invalid index", $n))
        };
    }
    macro_rules! command {
        ($r:expr) => {
            machine
                .program
                .get(machine.instruction_pointer..=machine.instruction_pointer + $r)
                .expect("opcode read out of bounds")
                .to_owned()
        };
    }

    let current_instruction = machine.program.get(machine.instruction_pointer);
    if let Some(opcode) = current_instruction {
        match opcode {
            1 => {
                let addition = command!(4);
                *position!(addition[3]) = *position!(addition[1]) + *position!(addition[2]);
                machine.instruction_pointer += 4;
            }
            2 => {
                let multiplication = command!(4);
                *position!(multiplication[3]) =
                    *position!(multiplication[1]) * *position!(multiplication[2]);
                machine.instruction_pointer += 4;
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
