#![warn(clippy::pedantic)]

use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("part 1: {}", part_1(&input));
    println!("part 1: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let initial_state = (HashSet::new(), (0, 0));

    input
        .trim()
        .chars()
        .map(map_to_direction)
        .fold(initial_state, |(mut visited, santa_pos), dir| {
            visited.insert(santa_pos);
            (visited, walk(santa_pos, &dir))
        })
        .0
        .len()
}

fn part_2(input: &str) -> usize {
    let initial_state = (HashSet::from([(0, 0)]), (0, 0), (0, 0));

    input
        .trim()
        .chars()
        .map(map_to_direction)
        .enumerate()
        .fold(
            initial_state,
            |(mut visited, santa_pos, robo_pos), (i, dir)| {
                let is_santas_turn = i % 2 == 0;

                if is_santas_turn {
                    let next_santa = walk(santa_pos, &dir);
                    visited.insert(next_santa);
                    return (visited, next_santa, robo_pos);
                }

                let next_robo = walk(robo_pos, &dir);
                visited.insert(next_robo);
                (visited, santa_pos, next_robo)
            },
        )
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
