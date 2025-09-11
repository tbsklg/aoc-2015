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
    parse_lines(input);
    0
}

fn parse_lines(input: &str) {
    let i = input.lines().map(parse_line).collect::<Vec<_>>();
    println!("{:?}", i);
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

    Instruction { action, range: (from, to) }
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
