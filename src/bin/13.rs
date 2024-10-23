use std::collections::HashMap;

use advent_of_code::{execute, parse_machine, Coordinate, State};
use itertools::Itertools;

advent_of_code::solution!(13);

#[allow(dead_code)]
fn render(screen: &HashMap<Coordinate<i64>, i64>) {
    let (_min_x, max_x) = screen
        .keys()
        .map(|l| l.left)
        .minmax()
        .into_option()
        .unwrap();
    let (_min_y, max_y) = screen.keys().map(|l| l.top).minmax().into_option().unwrap();

    for top in 0..=max_y {
        for left in 0..=max_x {
            print!(
                "{}",
                match screen.get(&Coordinate { left, top }).unwrap_or(&0) {
                    0 => ".",
                    1 => "|",
                    2 => "#",
                    3 => "-",
                    4 => "o",
                    _ => "?",
                }
            );
        }
        println!();
    }
    println!();
    println!(
        "score: {}",
        screen
            .get(&Coordinate {
                left: -1_i64,
                top: 0_i64
            })
            .unwrap_or(&-999_i64)
    );
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, mut machine) = parse_machine(input).unwrap();

    execute(&mut machine);

    let mut screen = HashMap::new();
    for (x, y, id) in machine.outputs.iter().tuples() {
        screen
            .entry(Coordinate { left: *x, top: *y })
            .and_modify(|k: &mut i64| *k = *id)
            .or_insert(*id);
    }

    Some(screen.values().filter(|c| **c == 2).count())
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, mut machine) = parse_machine(input).unwrap();
    let mut screen = HashMap::new();

    machine.program[0] = 2;
    loop {
        // will either be waiting on input or terminated
        let state = execute(&mut machine);
        // but we need to update the screen either way
        for (x, y, id) in machine.outputs.iter().tuples() {
            screen
                .entry(Coordinate { left: *x, top: *y })
                .and_modify(|k: &mut i64| *k = *id)
                .or_insert(*id);
        }

        // then exit if it's done
        if matches!(state, State::Terminated) {
            break;
        }

        // keep the paddle under the ball
        let ball_x = screen.iter().find(|(_, v)| **v == 4).unwrap().0.left;
        let paddle_x = screen.iter().find(|(_, v)| **v == 3).unwrap().0.left;

        machine.inputs.push_back(match ball_x - paddle_x {
            x if x < 0 => -1,
            x if x > 0 => 1,
            _ => 0,
        });
    }

    Some(*screen.get(&Coordinate { left: -1, top: 0 }).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
