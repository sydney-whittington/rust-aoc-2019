use std::collections::HashMap;

use advent_of_code::CoordinateSigned;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, u32},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

advent_of_code::solution!(3);

#[derive(Debug)]
enum Turn {
    U(u32),
    R(u32),
    D(u32),
    L(u32),
}

enum Panel {
    A,
    B,
    Cross,
}

fn turn_from_char(c: &str, n: u32) -> Turn {
    match c {
        "U" => Turn::U(n),
        "R" => Turn::R(n),
        "D" => Turn::D(n),
        "L" => Turn::L(n),
        _ => panic!("not a turn"),
    }
}

fn parser(i: &str) -> IResult<&str, (Vec<Turn>, Vec<Turn>)> {
    let (i, a) = separated_list1(tag(","), tuple((alpha1, u32)))(i)?;
    let a = a.iter().map(|(c, n)| turn_from_char(c, *n)).collect();
    let (i, b) = preceded(newline, separated_list1(tag(","), tuple((alpha1, u32))))(i)?;
    let b = b.iter().map(|(c, n)| turn_from_char(c, *n)).collect();

    Ok((i, (a, b)))
}

pub fn part_one(input: &str) -> Option<i32> {
    let (_, (wire_a, wire_b)) = parser(input).unwrap();

    let mut paths = HashMap::new();

    let mut location = CoordinateSigned { x: 0, y: 0 };
    for turn in wire_a {
        match turn {
            Turn::U(n) => {
                for _ in 0..n {
                    location.y += 1;
                    paths.insert(location, Panel::A);
                }
            }
            Turn::R(n) => {
                for _ in 0..n {
                    location.x += 1;
                    paths.insert(location, Panel::A);
                }
            }
            Turn::D(n) => {
                for _ in 0..n {
                    location.y -= 1;
                    paths.insert(location, Panel::A);
                }
            }
            Turn::L(n) => {
                for _ in 0..n {
                    location.x -= 1;
                    paths.insert(location, Panel::A);
                }
            }
        }
    }

    location = CoordinateSigned { x: 0, y: 0 };
    for turn in wire_b {
        match turn {
            Turn::U(n) => {
                for _ in 0..n {
                    location.y += 1;
                    paths
                        .entry(location)
                        .and_modify(|p| {
                            if matches!(p, Panel::A) {
                                *p = Panel::Cross
                            }
                        })
                        .or_insert(Panel::B);
                }
            }
            Turn::R(n) => {
                for _ in 0..n {
                    location.x += 1;
                    paths
                        .entry(location)
                        .and_modify(|p| {
                            if matches!(p, Panel::A) {
                                *p = Panel::Cross
                            }
                        })
                        .or_insert(Panel::B);
                }
            }
            Turn::D(n) => {
                for _ in 0..n {
                    location.y -= 1;
                    paths
                        .entry(location)
                        .and_modify(|p| {
                            if matches!(p, Panel::A) {
                                *p = Panel::Cross
                            }
                        })
                        .or_insert(Panel::B);
                }
            }
            Turn::L(n) => {
                for _ in 0..n {
                    location.x -= 1;
                    paths
                        .entry(location)
                        .and_modify(|p| {
                            if matches!(p, Panel::A) {
                                *p = Panel::Cross
                            }
                        })
                        .or_insert(Panel::B);
                }
            }
        }
    }

    let closest_cross = paths
        .iter()
        .filter(|(_, p)| matches!(p, Panel::Cross))
        .map(|(c, _)| c.x.abs() + c.y.abs())
        .min()
        .unwrap();
    Some(closest_cross)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(159));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
