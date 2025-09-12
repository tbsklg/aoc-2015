use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let now = std::time::Instant::now();
    println!("part 1: {} ({:?})", part_1(&input), now.elapsed());
}

#[derive(Debug, PartialEq, Eq)]
enum Action {
    Turn(bool),
    Toggle,
}

type Range = ((usize, usize), (usize, usize));

#[derive(Debug)]
struct Instruction {
    range: Range,
    action: Action,
}

fn part_1(input: &str) -> usize {
    // TODO: Improve performance
    let i = input
        .lines()
        .map(parse_line)
        .fold(HashSet::new(), |mut acc, instr| {
            println!("{:?}", instr);
            for x in instr.range.0.0..=instr.range.1.0 {
                for y in instr.range.0.1..=instr.range.1.1 {
                    match instr.action {
                        Action::Turn(true) => {
                            acc.insert((x, y));
                        }
                        Action::Turn(false) => {
                            acc.remove(&(x, y));
                        }
                        Action::Toggle => {
                            if !acc.insert((x, y)) {
                                acc.remove(&(x, y));
                            }
                        }
                    }
                }
            }
            acc
        });

    i.len()
}

fn parse_line(line: &str) -> Instruction {
    let mut parts = line.split(' ');

    let action = match parts.next().unwrap() {
        "turn" => {
            let on_off = parts.next().unwrap();

            match on_off {
                "on" => Action::Turn(true),
                "off" => Action::Turn(false),
                _ => unimplemented!(),
            }
        }
        "toggle" => Action::Toggle,
        _ => unimplemented!(),
    };

    let mut iter = parts.next().unwrap().split(',');
    let from = (
        iter.next().unwrap().parse::<usize>().unwrap(),
        iter.next().unwrap().parse::<usize>().unwrap(),
    );

    parts.next();

    let mut iter = parts.next().unwrap().split(',');
    let to = (
        iter.next().unwrap().parse::<usize>().unwrap(),
        iter.next().unwrap().parse::<usize>().unwrap(),
    );

    Instruction {
        action,
        range: (from, to),
    }
}

#[cfg(test)]
mod tests {
    use crate::{Action, parse_line};

    #[test]
    fn should_parse_toggle_line() {
        let line = "toggle 461,550 through 564,900";
        let instruction = parse_line(line);
        assert_eq!(instruction.action, Action::Toggle);
        assert_eq!(instruction.range, ((461, 550), (564, 900)));
    }

    #[test]
    fn should_parse_turn_on_line() {
        let line = "turn on 370,39 through 425,839";
        let instruction = parse_line(line);
        assert_eq!(instruction.action, Action::Turn(true));
        assert_eq!(instruction.range, ((370, 39), (425, 839)));
    }

    #[test]
    fn should_parse_turn_off_line() {
        let line = "turn off 464,858 through 833,915";
        let instruction = parse_line(line);
        assert_eq!(instruction.action, Action::Turn(false));
        assert_eq!(instruction.range, ((464, 858), (833, 915)));
    }
}
