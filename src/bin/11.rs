use std::collections::HashMap;

use advent_of_code::{execute, parse_machine, Coordinate, State};
use itertools::Itertools;

advent_of_code::solution!(11);

enum Facing {
    Up,
    Right,
    Down,
    Left,
}

struct Robot {
    facing: Facing,
    location: Coordinate<i32>,
}

fn update_robot(robot: &Robot, direction: i64) -> Robot {
    let facing = match direction {
        // left
        0 => match robot.facing {
            Facing::Up => Facing::Left,
            Facing::Right => Facing::Up,
            Facing::Down => Facing::Right,
            Facing::Left => Facing::Down,
        },
        // right
        1 => match robot.facing {
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
        },
        _ => panic!("non turn value"),
    };

    let location = match facing {
        Facing::Up => Coordinate {
            left: robot.location.left,
            top: robot.location.top - 1,
        },
        Facing::Right => Coordinate {
            left: robot.location.left + 1,
            top: robot.location.top,
        },
        Facing::Down => Coordinate {
            left: robot.location.left,
            top: robot.location.top + 1,
        },
        Facing::Left => Coordinate {
            left: robot.location.left - 1,
            top: robot.location.top,
        },
    };

    Robot { facing, location }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, mut machine) = parse_machine(input).unwrap();

    let mut panel = HashMap::new();
    let mut robot = Robot {
        facing: Facing::Up,
        location: Coordinate { left: 0, top: 0 },
    };

    while matches!(execute(&mut machine), State::WaitingForInput) {
        for (color, direction) in machine.outputs.drain(..).tuple_windows() {
            panel.insert(robot.location, color);
            robot = update_robot(&robot, direction);
        }
        machine
            .inputs
            .push_back(*panel.get(&robot.location).unwrap_or(&0_i64));
    }

    Some(panel.len())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // no part 1 tests

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
