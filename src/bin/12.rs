use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};
use num::integer::lcm;

advent_of_code::solution!(12);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Moon {
    position: (i32, i32, i32),
    velocity: (i32, i32, i32),
}

impl Moon {
    fn potential(&self) -> i32 {
        self.position.0.abs() + self.position.1.abs() + self.position.2.abs()
    }

    fn kinetic(&self) -> i32 {
        self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs()
    }

    fn total(&self) -> i32 {
        self.potential() * self.kinetic()
    }

    fn update_velocity(&mut self, other: (i32, i32, i32)) {
        self.velocity.0 += match self.position.0.cmp(&other.0) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
        self.velocity.1 += match self.position.1.cmp(&other.1) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
        self.velocity.2 += match self.position.2.cmp(&other.2) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
    }

    fn update_position(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }

    fn get_dimensions(&self) -> ((i32, i32), (i32, i32), (i32, i32)) {
        (
            (self.position.0, self.velocity.0),
            (self.position.1, self.velocity.1),
            (self.position.2, self.velocity.2),
        )
    }
}

fn parse_moon(i: &str) -> IResult<&str, Moon> {
    let (i, x) = preceded(tag("<x="), i32)(i)?;
    let (i, y) = preceded(tag(", y="), i32)(i)?;
    let (i, z) = delimited(tag(", z="), i32, tag(">"))(i)?;

    Ok((
        i,
        Moon {
            position: (x, y, z),
            velocity: (0, 0, 0),
        },
    ))
}

fn parser(i: &str) -> IResult<&str, Vec<Moon>> {
    separated_list1(newline, parse_moon)(i)
}

fn simulate(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            let other_position = moons[j].position;
            moons[i].update_velocity(other_position);
        }
    }

    moons.iter_mut().for_each(|m| m.update_position());
}

pub fn part_one(input: &str) -> Option<i32> {
    let (_, mut moons) = parser(input).unwrap();

    for _ in 0..1000 {
        simulate(&mut moons);
    }
    Some(moons.iter().map(|m| m.total()).sum::<i32>())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, mut moons) = parser(input).unwrap();
    let mut x_seen = HashMap::new();
    let mut y_seen = HashMap::new();
    let mut z_seen = HashMap::new();

    let mut x_cycle = 0;
    let mut y_cycle = 0;
    let mut z_cycle = 0;

    for i in 0.. {
        let (x, y, z) = moons.iter().map(|m| m.get_dimensions()).fold(
            (vec![], vec![], vec![]),
            |(mut xs, mut ys, mut zs), (x, y, z)| {
                xs.push(x);
                ys.push(y);
                zs.push(z);
                (xs, ys, zs)
            },
        );
        // is all this repetition really the best we can do?
        if x_cycle == 0 {
            if let Some(t) = x_seen.insert(x, i) {
                x_cycle = i - t;
            }
        }
        if y_cycle == 0 {
            if let Some(t) = y_seen.insert(y, i) {
                y_cycle = i - t;
            }
        }
        if z_cycle == 0 {
            if let Some(t) = z_seen.insert(z, i) {
                z_cycle = i - t;
            }
        }

        simulate(&mut moons);

        if x_cycle != 0 && y_cycle != 0 && z_cycle != 0 {
            return Some(lcm(x_cycle, lcm(y_cycle, z_cycle)));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let (_, mut moons) = parser(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        for _ in 0..10 {
            simulate(&mut moons);
        }
        assert_eq!(moons.iter().map(|m| m.total()).sum::<i32>(), 179);
    }

    #[test]
    fn test_part_one_b() {
        let (_, mut moons) = parser(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ))
        .unwrap();
        for _ in 0..100 {
            simulate(&mut moons);
        }
        assert_eq!(moons.iter().map(|m| m.total()).sum::<i32>(), 1940);
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2772));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4686774924));
    }
}
