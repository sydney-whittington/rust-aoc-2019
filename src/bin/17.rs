use std::collections::HashMap;

use advent_of_code::{execute, parse_machine, Coordinate};

advent_of_code::solution!(17);

enum Facing {
    Up,
    Right,
    Down,
    Left,
}

#[allow(dead_code)]
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
    // print!("{chars}");
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

pub fn part_two(input: &str) -> Option<i64> {
    let (_, mut machine) = parse_machine(input).unwrap();
    machine.program[0] = 2;

    // hand solved in data/examples/17-solved.txt
    let main = "A,B,A,C,B,C,A,B,A,C\n";
    let a = "R,6,L,10,R,8,R,8\n";
    let b = "R,12,L,8,L,10\n";
    let c = "R,12,L,10,R,6,L,10\n";
    let live = "n\n";

    let routine = format!("{}{}{}{}{}", main, a, b, c, live);
    machine
        .inputs
        .extend(routine.as_bytes().iter().map(|c| *c as i64));

    execute(&mut machine);

    Some(*machine.outputs.back().unwrap())
}

// no tests (technically there are examples but they don't work on the same interface)
