use advent_of_code::{execute, parse_machine};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i64> {
    let (_, mut machine) = parse_machine(input).unwrap();

    machine.inputs.push_back(1);
    execute(&mut machine);
    dbg!(&machine.outputs);

    // last value is the diagnostic code
    machine.outputs.pop_back()
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, mut machine) = parse_machine(input).unwrap();

    machine.inputs.push_back(5);
    execute(&mut machine);
    machine.outputs.pop_front()
}

// no tests
