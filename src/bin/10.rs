use std::collections::{HashMap, HashSet};

use advent_of_code::Coordinate;
use itertools::Itertools;
use num::Integer;

advent_of_code::solution!(10);

enum Space {
    Empty,
    Asteroid,
}

// could just use a hashset if we only have one thing we're tracking, pending part 2
type Region = HashMap<Coordinate<usize>, Space>;

// manhattan distance is fine because we're just looking to order points intelligently
fn distance(a: &Coordinate<usize>, b: &Coordinate<usize>) -> usize {
    a.left.abs_diff(b.left) + a.top.abs_diff(b.top)
}

fn parser(i: &str) -> Region {
    let mut region = Region::new();
    for (top, line) in i.lines().enumerate() {
        for (left, character) in line.chars().enumerate() {
            match character {
                '#' => {
                    region.insert(Coordinate { left, top }, Space::Asteroid);
                }
                '.' => {
                    ();
                    // no need to track empty values if we can't build on them
                    // region.insert(Coordinate { left, top }, Space::Empty);
                }
                _ => {
                    panic!("unexpected character");
                }
            }
        }
    }

    region
}

fn visible(origin: &Coordinate<usize>, region: &Region) -> usize {
    let mut seen = HashSet::new();
    for location in region.keys().sorted_by_key(|l| distance(origin, l)) {
        // don't count yourself
        if location == origin {
            continue;
        }

        let x_vec = origin.left as isize - location.left as isize;
        let y_vec = origin.top as isize - location.top as isize;
        let gcd =  x_vec.gcd(&y_vec);

        // reduce the fraction and store the differential
        // overlaps will be dropped and we'll only store unique vector directions
        seen.insert((x_vec/gcd, y_vec/gcd));
    }
    
    seen.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let region = parser(input);


    
    let best = region.keys().max_by_key(|l| visible(l, &region)).unwrap();
    let value = visible(best, &region);
    println!("best location is {} with {}", best, value);

    Some(value)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(210));
    }

    #[test]
    fn test_visibility() {
        let region = parser(&advent_of_code::template::read_file_part("examples", DAY, 1));
        let result = visible(&Coordinate { left: 0_usize, top: 0_usize }, &region);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
