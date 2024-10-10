use nom::{
    bytes::complete::tag,
    character::complete::{digit1, i32},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

use std::{
    io::{self, Write},
    str::FromStr,
};
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
    Placeholder,
}

fn mode_from_digit(c: &char) -> Mode {
    match c {
        '0' => Mode::Position,
        '1' => Mode::Immediate,
        _ => panic!("unexpected mode {}", c),
    }
}

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Addition,
    Multiplication,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

fn opcode_from_number(n: &i32) -> Opcode {
    match n {
        1 => Opcode::Addition,
        2 => Opcode::Multiplication,
        3 => Opcode::Input,
        4 => Opcode::Output,
        5 => Opcode::JumpIfTrue,
        6 => Opcode::JumpIfFalse,
        7 => Opcode::LessThan,
        8 => Opcode::Equals,
        99 => Opcode::Halt,
        _ => unimplemented!("received unknown opcode {}", n),
    }
}

struct Instruction {
    opcode: Opcode,
    modes: (Mode, Mode, Mode, Mode),
}

fn decode_instruction(instruction: &i32) -> Instruction {
    let opcode = opcode_from_number(&(instruction % 100));
    let digits = format!("{:0>5}", instruction.to_string())
        .chars()
        .collect::<Vec<_>>();
    let mode_1 = mode_from_digit(&digits[2]);
    let mode_2 = mode_from_digit(&digits[1]);
    let mode_3 = mode_from_digit(&digits[0]);

    Instruction {
        opcode,
        modes: (Mode::Placeholder, mode_1, mode_2, mode_3),
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
            match $mode {
                Mode::Position => *position!($n),
                Mode::Immediate => $n,
                _ => unimplemented!(),
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
        let instruction = decode_instruction(instruction);
        match instruction.opcode {
            Opcode::Addition => {
                let parameters = command!(4);
                *position!(parameters[3]) = value!(parameters[1], instruction.modes.1)
                    + value!(parameters[2], instruction.modes.2);
            }
            Opcode::Multiplication => {
                let parameters = command!(4);
                *position!(parameters[3]) = value!(parameters[1], instruction.modes.1)
                    * value!(parameters[2], instruction.modes.2);
            }
            Opcode::Input => {
                let parameters = command!(2);
                println!("Input: ");
                io::stdout().flush().expect("unable to flush stdout");

                let mut input_line = String::new();
                io::stdin()
                    .read_line(&mut input_line)
                    .expect("Failed to read line");
                let input: i32 = input_line.trim().parse().expect("Input not an integer");
                *position!(parameters[1]) = input;
            }
            Opcode::Output => {
                // TODO: save outputs to test programatically?
                let parameters = command!(2);
                println!("Output: {}", value!(parameters[1], instruction.modes.1));
            }
            Opcode::JumpIfTrue => {
                let parameters = command!(3);
                if value!(parameters[1], instruction.modes.1) != 0 {
                    machine.instruction_pointer = value!(parameters[2], instruction.modes.2)
                        .try_into()
                        .expect("attempt to jump to invalid address");
                }
            }
            Opcode::JumpIfFalse => {
                let parameters = command!(3);
                if value!(parameters[1], instruction.modes.1) == 0 {
                    machine.instruction_pointer = value!(parameters[2], instruction.modes.2)
                        .try_into()
                        .expect("attempt to jump to invalid address");
                }
            }
            Opcode::LessThan => {
                let parameters = command!(4);
                *position!(parameters[3]) = (value!(parameters[1], instruction.modes.1)
                    < value!(parameters[2], instruction.modes.2))
                    as i32;
            }
            Opcode::Equals => {
                let parameters = command!(4);
                *position!(parameters[3]) = (value!(parameters[1], instruction.modes.1)
                    == value!(parameters[2], instruction.modes.2))
                    as i32;
            }
            Opcode::Halt => {
                // command!(1);
                return false;
            } // _ => unimplemented!(),
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
