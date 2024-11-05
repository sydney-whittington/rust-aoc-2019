use std::collections::{HashMap, VecDeque};

use advent_of_code::{execute, parse_machine, IntcodeMachine};
use enum_iterator::{all, Sequence};
use itertools::Itertools;

advent_of_code::solution!(15);

#[derive(Sequence, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
enum Terrain {
    Wall,
    Floor,
    Oxygen,
}

type Location = (i64, i64);
type Map = HashMap<Location, Terrain>;

fn to_command(direction: &Direction) -> i64 {
    match direction {
        Direction::North => 1,
        Direction::South => 2,
        Direction::West => 3,
        Direction::East => 4,
    }
}

fn transform_location(direction: &Direction, location: &Location) -> Location {
    match direction {
        Direction::North => (location.0, location.1 + 1),
        Direction::East => (location.0 + 1, location.1),
        Direction::South => (location.0, location.1 - 1),
        Direction::West => (location.0 - 1, location.1),
    }
}

fn from_status_code(code: &i64) -> Terrain {
    match code {
        0 => Terrain::Wall,
        1 => Terrain::Floor,
        2 => Terrain::Oxygen,
        _ => panic!("unexpected status code"),
    }
}

fn visualize(map: &Map) {
    let (min_x, max_x) = map.keys().map(|l| l.0).minmax().into_option().unwrap();
    let (min_y, max_y) = map.keys().map(|l| l.1).minmax().into_option().unwrap();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            print!(
                "{}",
                match map.get(&(x, y)) {
                    Some(Terrain::Floor) => ".",
                    Some(Terrain::Wall) => "#",
                    Some(Terrain::Oxygen) => "!",
                    None => " ",
                }
            );
        }
        println!();
    }
    println!();
}


fn explore(machine: &mut IntcodeMachine, map: &mut Map, location: Location) {
    let mut stack = VecDeque::from([(machine.clone(), location)]);

    while let Some((machine, location)) = stack.pop_front() {
        for direction in all::<Direction>() {
            let new_location = transform_location(&direction, &location);
            if map.contains_key(&new_location) {
                continue;
            }

            let mut branch_machine = machine.clone();
            branch_machine.inputs.push_back(to_command(&direction));
            execute(&mut branch_machine);

            let terrain = from_status_code(&branch_machine.outputs.pop_front().unwrap());
            map.insert(new_location, terrain);

            match terrain {
                // recurse
                Terrain::Floor | Terrain::Oxygen => {
                    stack.push_back((branch_machine, new_location));
                },
                // dead end, stop exploring that path
                Terrain::Wall => (),
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut machine) = parse_machine(input).unwrap();
    let mut map = HashMap::from([((0, 0), Terrain::Floor)]);

    explore(&mut machine, &mut map, (0, 0));

    visualize(&map);

    // TODO: then actually get to the thing from the built map

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
