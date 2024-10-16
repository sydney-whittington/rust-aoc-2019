use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug)]
struct Image {
    layers: Vec<Layer>
}

#[derive(Debug)]
struct Layer {
    content: HashMap<(usize, usize), u32>
}

fn parser(i: &str, (wide, tall): (usize, usize)) -> Image {
    let digits = i.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
    let mut layers = Vec::new();
    for chunk in digits.into_iter().chunks(wide*tall).into_iter() {
        let mut layer = Layer { content: HashMap::new()};
        for (digit, (row, column)) in chunk.zip((0..tall).cartesian_product(0..wide)) {
            layer.content.insert((row, column), digit);
        }
        layers.push(layer); 
    }

    Image { layers }
}

fn corruption_check(image: Image) -> usize {
    let fewest_zeroes = image.layers.iter().min_by_key(|l| l.content.values().filter(|n| **n == 0).count()).unwrap();

    fewest_zeroes.content.values().filter(|n| **n == 1).count() * fewest_zeroes.content.values().filter(|n| **n == 2).count()
}

pub fn part_one(input: &str) -> Option<usize> {
    let image = parser(input, (25, 6));

    Some(corruption_check(image))
}

pub fn part_two(input: &str) -> Option<u32> {
    let image = parser(input, (25, 6));
    let mut rendered = HashMap::new();

    for layer in image.layers.iter().rev() {
        for (location, value) in layer.content.iter() {
            rendered.entry(location).and_modify(|p| *p = *match value {
                2 => p,
                1 | 0 => value,
                _ => panic!("got a non-expected value"),
            }).or_insert(*value);
        }
    }

    for row in 0..6 {
        for column in 0..25 {
            print!("{}", match rendered.get(&(row, column)).unwrap() {
                1 => "*",
                0 => " ",
                _ => panic!("unexpected character"),
            });
        }
        println!();
    }
    println!();

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let image = parser(&advent_of_code::template::read_file("examples", DAY), (3, 2));
        let result = Some(corruption_check(image));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
