use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use petgraph::{
    algo::dijkstra,
    prelude::{DiGraphMap, UnGraphMap},
};

advent_of_code::solution!(6);

fn parser(i: &str) -> IResult<&str, Vec<(&str, &str)>> {
    let (i, edges) = separated_list1(
        newline,
        separated_pair(alphanumeric1, tag(")"), alphanumeric1),
    )(i)?;

    Ok((i, edges))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, edges) = parser(input).unwrap();

    let mut graph = DiGraphMap::<&str, ()>::new();
    graph.add_node("COM");
    for edge in edges {
        graph.add_node(edge.1);
        graph.add_edge(edge.1, edge.0, ());
    }

    let distances = graph
        .nodes()
        .map(|n| {
            *dijkstra(&graph, n, Some("COM"), |_| 1_u32)
                .get("COM")
                .unwrap()
        })
        .sum();

    Some(distances)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, edges) = parser(input).unwrap();

    let mut graph = UnGraphMap::<&str, ()>::new();
    graph.add_node("COM");
    for edge in edges {
        graph.add_node(edge.1);
        graph.add_edge(edge.1, edge.0, ());
    }

    let distance = *dijkstra(&graph, "YOU", Some("SAN"), |_| 1_u32)
        .get("SAN")
        .unwrap();
    // -2 since we're not transferring from ourselves or to the SAN
    Some(distance - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }
}
