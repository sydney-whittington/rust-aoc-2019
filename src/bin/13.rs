use std::collections::HashMap;

use advent_of_code::{execute, parse_machine, Coordinate};
use itertools::Itertools;

advent_of_code::solution!(13);

fn render(panel: &HashMap<Coordinate<i64>, i64>) {
    let (min_x, max_x) = panel.keys().map(|l| l.left).minmax().into_option().unwrap();
    let (min_y, max_y) = panel.keys().map(|l| l.top).minmax().into_option().unwrap();

    for top in min_y..=max_y {
        for left in min_x..=max_x {
            print!(
                "{}",
                match panel.get(&Coordinate { left, top }).unwrap_or(&0) {
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

    render(&screen);

    Some(screen.values().filter(|c| **c == 2).count())
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
