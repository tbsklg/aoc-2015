use std::{collections::HashMap, iter::Map};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let now = std::time::Instant::now();
    println!("part 1: {} ({:?})", part_1(&input), now.elapsed());
}

fn part_1(input: &str) -> usize {
    let distances = parse_distances(input).unwrap();
    println!("{:?}", distances);
    0
}

fn parse_distances(
    input: &str,
) -> Result<HashMap<(String, String), usize>, Box<dyn std::error::Error>> {
    input
        .lines()
        .map(parse_distance)
        .collect::<Result<HashMap<_, _>, _>>()
}

fn parse_distance(input: &str) -> Result<((String, String), usize), Box<dyn std::error::Error>> {
    let mut parts = input.split(" to ").flat_map(|s| s.split(" = "));

    let from = parts.next().ok_or("Missing first city")?;
    let to = parts.next().ok_or("Missing second city")?;
    let distance = parts.next().ok_or("Missing distance")?;
    let distance = distance.parse::<usize>()?;

    Ok(((from.to_string(), to.to_string()), distance))
}
