#![warn(clippy::pedantic)]

use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("part 1: {}", part_1(&input));
}

fn part_1(input: &str) -> usize {
    let initial_state = (HashSet::new(), (0, 0));

    input
        .trim()
        .chars()
        .map(map_to_direction)
        .fold(initial_state, |(mut visited, curr), dir| {
            visited.insert(curr);
            (visited, walk(curr, &dir))
        })
        .0
        .len()
}

type Pos = (i64, i64);

#[derive(Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}

fn walk((x, y): Pos, dir: &Dir) -> Pos {
    match dir {
        Dir::U => (x, y + 1),
        Dir::D => (x, y - 1),
        Dir::L => (x - 1, y),
        Dir::R => (x + 1, y),
    }
}

fn map_to_direction(x: char) -> Dir {
    match x {
        '^' => Dir::U,
        'v' => Dir::D,
        '<' => Dir::L,
        '>' => Dir::R,
        _ => panic!("unknown direction"),
    }
}
