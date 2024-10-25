use std::collections::{HashMap, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, u32},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

advent_of_code::solution!(14);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Reactant {
    symbol: String,
    quantity: u32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Rule {
    precursors: Vec<Reactant>,
    product: Reactant,
}

type Reactions = HashMap<String, Rule>;

fn parse_rule(i: &str) -> IResult<&str, Rule> {
    let (i, precursors) = separated_list1(tag(", "), separated_pair(u32, tag(" "), alpha1))(i)?;
    let (i, product) = preceded(tag(" => "), separated_pair(u32, tag(" "), alpha1))(i)?;

    Ok((
        i,
        Rule {
            precursors: precursors
                .into_iter()
                .map(|(n, p)| Reactant {
                    symbol: p.to_owned(),
                    quantity: n,
                })
                .collect(),
            product: Reactant {
                symbol: product.1.to_owned(),
                quantity: product.0,
            },
        },
    ))
}

fn parser(i: &str) -> IResult<&str, Reactions> {
    let (i, rules) = separated_list1(newline, parse_rule)(i)?;

    Ok((
        i,
        HashMap::from_iter(rules.into_iter().map(|r| (r.product.symbol.clone(), r))),
    ))
}

fn produce<'a>(
    rules: &'a HashMap<String, Rule>,
    mut needed: VecDeque<(u32, &'a str)>,
    leftovers: &mut HashMap<&'a str, u32>,
) -> u32 {
    let mut ore = 0;

    while let Some((mut quantity_needed, chemical)) = needed.pop_front() {
        let rule = &rules[chemical];

        if let Some(stored) = leftovers.get(chemical) {
            if *stored >= quantity_needed {
                // println!("needed {quantity_needed} {chemical} and had it in stock");
                leftovers
                    .entry(chemical)
                    .and_modify(|s| *s -= quantity_needed);
                continue;
            } else {
                quantity_needed -= stored;
                leftovers.remove(chemical);
            }
        }
        // print!("need {quantity_needed} {chemical}: from ");

        let reactions_needed = quantity_needed.div_ceil(rule.product.quantity);
        for precursor in rule.precursors.iter() {
            let precursor_needed = reactions_needed * precursor.quantity;
            // print!("{} {}, ", precursor_needed, precursor.symbol);
            if precursor.symbol == "ORE" {
                ore += precursor_needed;
            } else {
                needed.push_back((precursor_needed, &precursor.symbol));
            }
        }
        let too_many = (reactions_needed * rule.product.quantity) - quantity_needed;
        // println!(" with {} leftovers", too_many);

        leftovers
            .entry(&rule.product.symbol)
            .and_modify(|p| *p += too_many)
            .or_insert(too_many);
    }

    ore
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, rules) = parser(input).unwrap();

    let needed = VecDeque::from([(1, "FUEL")]);
    let mut leftovers: HashMap<&str, u32> = HashMap::new();

    let ore_needed = produce(&rules, needed, &mut leftovers);
    dbg!(leftovers);

    Some(ore_needed)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, rules) = parser(input).unwrap();

    let mut leftovers: HashMap<&str, u32> = HashMap::new();
    let ore_for_one_fuel = produce(&rules, VecDeque::from([(1, "FUEL")]), &mut leftovers);

    // how many can we make easily?
    let fast_forward_factor: u64 = 1_000_000_000_000 / ore_for_one_fuel as u64;
    dbg!(&fast_forward_factor);
    // it looks like we can be more efficient considering leftovers from the first time
    leftovers
        .values_mut()
        .for_each(|l| *l *= fast_forward_factor as u32);
    let mut remaining_ore = 1_000_000_000_000 - ore_for_one_fuel as u64 * fast_forward_factor;
    let mut total_fuel = fast_forward_factor;

    loop {
        match remaining_ore.checked_sub(produce(
            &rules,
            VecDeque::from([(1, "FUEL")]),
            &mut leftovers,
        ) as u64)
        {
            Some(n) => {
                remaining_ore = n;
                total_fuel += 1;
            }
            None => {
                return Some(total_fuel);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(165));
    }

    #[test]
    fn test_part_one_c() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2210736));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(460664));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(82892753));
    }
}
