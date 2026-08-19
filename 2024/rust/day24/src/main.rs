use std::collections::{HashMap, HashSet};

fn main() {
    static INPUT: &str = include_str!("../../../day24.txt");

    let mut values = HashMap::new();
    let (inputs, gates) = INPUT.split_once("\n\n").unwrap();
    for input in inputs.lines() {
        let (name, value) = input.split_once(": ").unwrap();
        let value = value.parse::<u64>().unwrap();
        values.insert(name, value);
    }
    let mut gates = gates
        .lines()
        .map(|gate| {
            let (expr, output) = gate.split_once(" -> ").unwrap();
            let (input1, expr) = expr.split_once(' ').unwrap();
            let (operation, input2) = expr.split_once(' ').unwrap();
            let operation = match operation {
                "AND" => GateOperation::And,
                "OR" => GateOperation::Or,
                "XOR" => GateOperation::Xor,
                _ => panic!(),
            };
            Gate {
                operation,
                input1,
                input2,
                output,
            }
        })
        .collect::<Vec<_>>();

    while !gates.is_empty() {
        let iter = gates.into_iter();
        gates = Vec::new();
        for gate in iter {
            if values.contains_key(&gate.input1) && values.contains_key(&gate.input2) {
                let input1 = *values.get(&gate.input1).unwrap();
                let input2 = *values.get(&gate.input2).unwrap();
                let output = match gate.operation {
                    GateOperation::And => input1 & input2,
                    GateOperation::Or => input1 | input2,
                    GateOperation::Xor => input1 ^ input2,
                };
                values.insert(gate.output, output);
            } else {
                gates.push(gate);
            }
        }
    }

    let mut part1 = 0;
    let mut i = 0;
    loop {
        if let Some(value) = values.get(format!("z{i:02}").as_str()) {
            part1 |= value << i;
        } else {
            break;
        }
        i += 1;
    }

    println!("{part1}");
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Gate {
    operation: GateOperation,
    input1: &'static str,
    input2: &'static str,
    output: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GateOperation {
    And,
    Or,
    Xor,
}
