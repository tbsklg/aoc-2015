use std::{collections::HashMap, process::exit};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let now = std::time::Instant::now();
    println!("part 1: {} ({:?})", part_1(&input).unwrap(), now.elapsed());
}

fn part_1(input: &str) -> Option<u16> {
    let gates = input
        .lines()
        .map(parse_gate)
        .collect::<HashMap<String, Gate>>();

    solve(gates)
}

fn solve(mut gates: HashMap<String, Gate>) -> Option<u16> {}

#[derive(Debug, Clone)]
enum Gate {
    Value(u16),
    Wire(String),
    Not(String),
    And(String, String),
    Or(String, String),
    Lshift(String, u8),
    Rshift(String, u8),
}

fn parse_gate(line: &str) -> (String, Gate) {
    let mut iter = line.split(" -> ");
    let mut left = iter.next().unwrap().split(" ");
    let gate = match left.next().unwrap() {
        "NOT" => Gate::Not(left.next().unwrap().to_string()),
        wire => {
            let wire = wire.to_string();
            if let Some(op) = left.next() {
                match op {
                    "RSHIFT" => {
                        let shift_amount = left.next().unwrap().parse().unwrap();
                        Gate::Rshift(wire, shift_amount)
                    }
                    "LSHIFT" => {
                        let shift_amount = left.next().unwrap().parse().unwrap();
                        Gate::Lshift(wire, shift_amount)
                    }
                    "AND" => {
                        let target_wire = left.next().unwrap().to_string();
                        Gate::And(wire, target_wire)
                    }
                    "OR" => {
                        let target_wire = left.next().unwrap().to_string();
                        Gate::Or(wire, target_wire)
                    }
                    xs => {
                        eprintln!("unknown operation: {xs}");
                        panic!("unknown operation");
                    }
                }
            } else {
                wire.parse::<u16>()
                    .map(Gate::Value)
                    .unwrap_or(Gate::Wire(wire))
            }
        }
    };
    let right = iter.next().unwrap().to_string();
    (right, gate)
}

fn shift_right(value: u16, amount: u8) -> u16 {
    value >> amount
}

fn shift_left(value: u16, amount: u8) -> u16 {
    value << amount
}

fn and(x: u16, y: u16) -> u16 {
    x & y
}

fn or(x: u16, y: u16) -> u16 {
    x | y
}

fn not(x: u16) -> u16 {
    !x
}

#[cfg(test)]
mod tests {
    use crate::shift_right;

    #[test]
    fn should_shift_right() {
        assert_eq!(shift_right(456, 2), 114);
    }

    #[test]
    fn should_shift_left() {
        assert_eq!(super::shift_left(456, 2), 1824);
    }

    #[test]
    fn should_and() {
        assert_eq!(super::and(456, 123), 72);
    }

    #[test]
    fn should_or() {
        assert_eq!(super::or(456, 123), 507);
    }
}
