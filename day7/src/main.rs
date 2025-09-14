use std::process::exit;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let now = std::time::Instant::now();
    println!("part 1: {} ({:?})", part_1(&input), now.elapsed());
}

fn part_1(input: &str) -> u16 {
    let xs = input.lines().map(parse_gate).collect::<Vec<_>>();
    println!("{:?}", xs);
    0
}
#[derive(Debug, Clone)]
enum Operand {
    Number(u16),
    Wire(String),
}

impl Operand {
    fn from_str(s: &str) -> Self {
        if let Ok(num) = s.parse::<u16>() {
            Operand::Number(num)
        } else {
            Operand::Wire(s.to_string())
        }
    }
}

#[derive(Debug, Clone)]
enum Gate {
    Value(Operand),
    Not(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Lshift(Operand, u8),
    Rshift(Operand, u8),
}

fn parse_gate(line: &str) -> (Gate, String) {
    let mut iter = line.split(" -> ");
    let mut left = iter.next().unwrap().split(" ");
    let gate = match left.next().unwrap() {
        "NOT" => {
            println!("NOT");
            let operand = left.next().unwrap();
            Gate::Not(Operand::from_str(operand))
        }
        other => {
            println!("other: {other}");
            if let Some(op) = left.next() {
                match op {
                    "RSHIFT" => {
                        let shift_amount = left.next().unwrap().parse().unwrap();
                        Gate::Rshift(Operand::from_str(other), shift_amount)
                    }
                    "LSHIFT" => {
                        let shift_amount = left.next().unwrap().parse().unwrap();
                        Gate::Lshift(Operand::from_str(other), shift_amount)
                    }
                    "AND" => {
                        let right_operand = left.next().unwrap();
                        Gate::And(Operand::from_str(other), Operand::from_str(right_operand))
                    }
                    "OR" => {
                        let right_operand = left.next().unwrap();
                        Gate::Or(Operand::from_str(other), Operand::from_str(right_operand))
                    }
                    xs => {
                        eprintln!("unknown operation: {xs}");
                        panic!("unknown operation");
                    }
                }
            } else {
                Gate::Value(Operand::from_str(other))
            }
        }
    };
    let right = iter.next().unwrap().to_string();
    (gate, right)
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
