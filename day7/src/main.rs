#![warn(clippy::pedantic)]

use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let now = std::time::Instant::now();
    println!("part 1: {} ({:?})", part_1(&input), now.elapsed());

    let now = std::time::Instant::now();
    println!("part 2: {} ({:?})", part_2(&input), now.elapsed());
}

fn part_1(input: &str) -> u16 {
    let gates = input
        .lines()
        .map(parse_gate)
        .collect::<HashMap<String, Gate>>();

    let mut mutable_gates = gates.clone();
    solve(&mut mutable_gates, "a")
}

fn part_2(input: &str) -> u16 {
    let gates = input
        .lines()
        .map(parse_gate)
        .collect::<HashMap<String, Gate>>();

    let mut mutable_gates = gates.clone();
    let result = solve(&mut mutable_gates, "a");

    let mut mutable_gates = gates.clone();
    mutable_gates.insert("b".to_string(), Gate::Value(result));
    solve(&mut mutable_gates, "a")
}

fn solve(gates: &mut HashMap<String, Gate>, wire: &str) -> u16 {
    if let Ok(num) = wire.parse::<u16>() {
        return num;
    }

    if let Some(gate) = gates.get(wire).cloned() {
        let value = match gate {
            Gate::Value(x) => x,
            Gate::Wire(w) => solve(gates, &w),
            Gate::Not(w) => !solve(gates, &w),
            Gate::And(w1, w2) => solve(gates, &w1) & solve(gates, &w2),
            Gate::Or(w1, w2) => solve(gates, &w1) | solve(gates, &w2),
            Gate::Lshift(w, amount) => solve(gates, &w) << amount,
            Gate::Rshift(w, amount) => solve(gates, &w) >> amount,
        };

        gates.insert(wire.to_string(), Gate::Value(value));
        value
    } else {
        panic!("Wire '{wire}' not found");
    }
}

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
    let mut left = iter.next().unwrap().split(' ');
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
