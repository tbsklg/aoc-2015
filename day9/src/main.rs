use anyhow::{Context, Result};
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

fn parse_distances(input: &str) -> Result<HashMap<(String, String), usize>> {
    Ok(input
        .lines()
        .map(parse_distance)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<HashMap<_, _>>())
}

fn parse_distance(input: &str) -> Result<Vec<((String, String), usize)>> {
    let mut parts = input.split(" to ").flat_map(|s| s.split(" = "));
    let from = parts.next().context("Missing first city")?;
    let to = parts.next().context("Missing second city")?;
    let distance = parts.next().context("Missing distance")?;
    let distance = distance
        .parse::<usize>()
        .context(format!("Invalid distance: {}", distance))?;

    Ok(vec![
        ((from.to_string(), to.to_string()), distance),
        ((to.to_string(), from.to_string()), distance),
    ])
}
