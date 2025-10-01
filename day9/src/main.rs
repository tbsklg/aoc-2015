use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let now = std::time::Instant::now();
    println!("part 1: {} ({:?})", part_1(&input), now.elapsed());

    let now = std::time::Instant::now();
    println!("part 2: {} ({:?})", part_2(&input), now.elapsed());
}

fn part_1(input: &str) -> usize {
    shortest_path(&parse_distances(input).unwrap())
}

fn part_2(input: &str) -> usize {
    let distances = parse_distances(input).unwrap();
    longest_path(&distances)
}

fn longest_path(distances: &HashMap<(String, String), usize>) -> usize {
    let all_cities: HashSet<String> = distances
        .keys()
        .flat_map(|(a, b)| vec![a.clone(), b.clone()])
        .collect();

    let total_cities = all_cities.len();
    let mut max_distance = 0;

    let mut stack = Vec::new();

    for start in &all_cities {
        let mut visited = HashSet::new();
        visited.insert(start.clone());
        stack.push((start.clone(), visited, 0));
    }

    while let Some((current, visited, dist)) = stack.pop() {
        if visited.len() == total_cities {
            max_distance = max_distance.max(dist);
            continue;
        }

        for next in &all_cities {
            if !visited.contains(next) {
                if let Some(&edge_dist) = distances.get(&(current.clone(), next.clone())) {
                    let mut new_visited = visited.clone();
                    new_visited.insert(next.clone());
                    stack.push((next.clone(), new_visited, dist + edge_dist));
                }
            }
        }
    }

    max_distance
}

fn shortest_path(distances: &HashMap<(String, String), usize>) -> usize {
    let all_cities: HashSet<String> = distances
        .keys()
        .flat_map(|(a, b)| vec![a.clone(), b.clone()])
        .collect();

    let total_cities = all_cities.len();
    let mut min_distance = usize::MAX;

    let mut stack = Vec::new();

    for start in &all_cities {
        let mut visited = HashSet::new();
        visited.insert(start.clone());
        stack.push((start.clone(), visited, 0));
    }

    while let Some((current, visited, dist)) = stack.pop() {
        if visited.len() == total_cities {
            min_distance = min_distance.min(dist);
            continue;
        }

        for next in &all_cities {
            if !visited.contains(next) {
                if let Some(&edge_dist) = distances.get(&(current.clone(), next.clone())) {
                    let mut new_visited = visited.clone();
                    new_visited.insert(next.clone());
                    stack.push((next.clone(), new_visited, dist + edge_dist));
                }
            }
        }
    }

    min_distance
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
