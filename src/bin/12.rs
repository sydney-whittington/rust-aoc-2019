use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

advent_of_code::solution!(12);

#[derive(Debug, Clone, Copy, PartialEq)]
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

pub fn part_two(_input: &str) -> Option<u32> {
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
