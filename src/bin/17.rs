use std::collections::HashMap;

use advent_of_code::{execute, parse_machine, Coordinate};
use itertools::Itertools;

advent_of_code::solution!(17);

enum Facing {
    Up,
    Right,
    Down,
    Left,
}

enum Location {
    Scaffold,
    Space,
    Robot(Facing),
}

pub fn adjacents(coord: Coordinate<usize>) -> Vec<Coordinate<usize>> {
    let mut result = Vec::from([
        Coordinate {
            left: coord.left + 1,
            top: coord.top,
        },
        Coordinate {
            left: coord.left,
            top: coord.top + 1,
        },
    ]);
    if coord.top > 0 {
        result.push(Coordinate {
            left: coord.left,
            top: coord.top - 1,
        });
    }
    if coord.left > 0 {
        result.push(Coordinate {
            left: coord.left - 1,
            top: coord.top,
        });
    }

    result
}

type Region = HashMap<Coordinate<usize>, Location>;

fn parse_region(i: &str) -> Region {
    let mut region = Region::new();
    for (top, line) in i.lines().enumerate() {
        for (left, character) in line.chars().enumerate() {
            match character {
                '#' => {
                    region.insert(Coordinate { left, top }, Location::Scaffold);
                }
                '.' => {
                    region.insert(Coordinate { left, top }, Location::Space);
                }
                '^' => {
                    region.insert(Coordinate { left, top }, Location::Robot(Facing::Up));
                }
                'v' => {
                    region.insert(Coordinate { left, top }, Location::Robot(Facing::Down));
                }
                '<' => {
                    region.insert(Coordinate { left, top }, Location::Robot(Facing::Left));
                }
                '>' => {
                    region.insert(Coordinate { left, top }, Location::Robot(Facing::Right));
                }
                _ => {
                    panic!("unexpected character");
                }
            }
        }
    }

    region
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, mut machine) = parse_machine(input).unwrap();

    execute(&mut machine);

    let chars: String = machine.outputs.iter().map(|c| (*c as u8) as char).collect();
    let region = parse_region(&chars);

    let intersections = region
        .iter()
        .filter(|(_, l)| matches!(l, Location::Scaffold))
        .filter(|(c, _)| {
            adjacents(**c)
                .iter()
                .map(|k| region.get(k).unwrap_or(&Location::Space))
                .all(|r| matches!(r, Location::Scaffold))
        });

    let alignments = intersections.map(|(c, _)| c.left * c.top);

    Some(alignments.sum())
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
