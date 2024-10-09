use nom::{character::complete::digit1, combinator::map_res, IResult};

use std::str::FromStr;
pub mod template;

// Use this file to add helper functions and additional modules.

pub fn parse_usize(i: &str) -> IResult<&str, usize> {
    map_res(digit1, usize::from_str)(i)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct CoordinateSigned {
    pub x: i32,
    pub y: i32,
}
