use itertools::Itertools;
use nalgebra::{Dyn, OMatrix};

advent_of_code::solution!(8);

#[derive(Debug)]
struct Image {
    layers: Vec<Layer>
}

#[derive(Debug)]
struct Layer {
    content: OMatrix<u32, Dyn, Dyn>
}

fn parser(i: &str, (wide, tall): (usize, usize)) -> Image {
    let digits = i.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
    let mut layers = Vec::new();
    for chunk in digits.into_iter().chunks(wide*tall).into_iter() {
        layers.push(Layer {content: OMatrix::<u32, Dyn, Dyn>::from_row_iterator(wide, tall, chunk)}); 
    }

    Image { layers }
}

fn corruption_check(image: Image) -> usize {
    let fewest_zeroes = image.layers.iter().min_by_key(|l| l.content.iter().filter(|n| **n == 0).count()).unwrap();

    fewest_zeroes.content.iter().filter(|n| **n == 1).count() * fewest_zeroes.content.iter().filter(|n| **n == 2).count()
}

pub fn part_one(input: &str) -> Option<usize> {
    let image = parser(input, (25, 6));

    Some(corruption_check(image))
}

pub fn part_two(_input: &str) -> Option<u32> {
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
