use std::{collections::VecDeque, iter};

use itertools::Itertools;

advent_of_code::solution!(16);

fn phase(nums: Vec<u32>) -> Vec<u32> {
    let mut output = Vec::new();
    let base_pattern: Vec<i32> = vec![0, 1, 0, -1];

    for i in 1..=nums.len() {
        let mod_pattern = base_pattern
            .iter()
            .flat_map(|v| iter::repeat_n(v, i))
            .cycle()
            .dropping(1);

        let inputs = nums.iter().zip(mod_pattern).collect_vec();

        let result = inputs.iter()
            .map(|(a, b)| **a as i32 * **b)
            .sum::<i32>()
            .unsigned_abs()
            % 10;

        output.push(result);
    }

    output
}

fn phase2(nums: VecDeque<u32>) -> VecDeque<u32> {
    let mut output = VecDeque::new();
    output.push_front(*nums.back().unwrap());

    for i in (0..nums.len()-1).rev() {
        let result = (nums[i] + *output.front().unwrap()) % 10;

        output.push_front(result);
    }

    output.into()
}
pub fn part_one(input: &str) -> Option<String> {
    let mut digits = input.chars().filter_map(|c| c.to_digit(10)).collect_vec();

    for _ in 0..100 {
        digits = phase(digits);
    }

    Some(digits.into_iter().take(8).join(""))
}

pub fn part_two(input: &str) -> Option<String> {
    let initial_digits = input.chars().filter_map(|c| c.to_digit(10)).collect_vec();
    let initial_length = initial_digits.len();
    let offset = initial_digits.iter().take(7).join("").parse::<usize>().unwrap();

    let mut digits = initial_digits.into_iter().cycle().take(initial_length * 10000).skip(offset).collect();

    for _ in 0..100 {
        digits = phase2(digits);
    }

    Some(digits.into_iter().take(8).join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let digits = "12345678".chars().filter_map(|c| c.to_digit(10)).collect_vec();
        let phase_1 = phase(digits);
        assert_eq!(phase_1.iter().join(""), "48226158".to_string());
        let phase_2 = phase(phase_1);
        assert_eq!(phase_2.iter().join(""), "34040438".to_string());
        let phase_3 = phase(phase_2);
        assert_eq!(phase_3.iter().join(""), "03415518".to_string());
        let phase_4 = phase(phase_3);
        assert_eq!(phase_4.iter().join(""), "01029498".to_string());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some("84462026".to_string()));
    }
}
