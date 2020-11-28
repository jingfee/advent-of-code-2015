use crate::solver::Solver;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

enum Operation {
    AND(String, String),
    OR(String, String),
    LSHIFT(String, u8),
    RSHIFT(String, u8),
    NOT(String),
    ASSIGN(u16),
    ASSIGNREF(String),
}

pub struct Instruction {
    operation: Operation,
    wire_output: String
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let line_split: Vec<&str> = line.split(' ').collect();
        let operation: Operation;
        let wire_output: String;
        if line_split.len() == 3 {
            let num = line_split[0].parse::<u16>();
            match num {
                Ok(val) => {
                    operation = Operation::ASSIGN(val);
                    wire_output = line_split[2].to_string()
                }
                Err(_err) => {
                    operation =
                        Operation::ASSIGNREF(line_split[0].to_string());
                    wire_output = line_split[2].to_string();
                }
            }
        } else if line_split.len() == 4 {
            operation = Operation::NOT(line_split[1].to_string());
            wire_output = line_split[3].to_string();
        } else {
            wire_output = line_split[4].to_string();
            match line_split[1] {
                "AND" => {
                    operation = Operation::AND(
                        line_split[0].to_string(),
                        line_split[2].to_string(),
                    )
                }
                "OR" => {
                    operation = Operation::OR(
                        line_split[0].to_string(),
                        line_split[2].to_string(),
                    )
                }
                "LSHIFT" => {
                    operation = Operation::LSHIFT(
                        line_split[0].to_string(),
                        line_split[2].to_string().parse::<u8>().unwrap(),
                    )
                }
                "RSHIFT" => {
                    operation = Operation::RSHIFT(
                        line_split[0].to_string(),
                        line_split[2].to_string().parse::<u8>().unwrap(),
                    )
                }
                _ => panic!("Unexpected input"),
            }
        }

        Instruction {
            operation: operation,
            wire_output: wire_output
        }
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Instruction>;
    type Output1 = u16;
    type Output2 = u16;

    fn parse_input(&self, file: File) -> Vec<Instruction> {
        let buf_reader = BufReader::new(file);
        buf_reader
            .lines()
            .map(|l| {
                let l = l.unwrap();
                Instruction::new(&l)
            })
            .collect()
    }

    fn solve_part_one(&self, input: &Vec<Instruction>) -> u16 {
        let mut wire_map: HashMap<String, u16> = HashMap::new();
        find_wire_signal(&input, &mut wire_map, "a")
    }

    fn solve_part_two(&self, input: &Vec<Instruction>) -> u16 {
        let signal_a = self.solve_part_one(input);
        let mut wire_map: HashMap<String, u16> = HashMap::new();
        wire_map.insert("b".to_string(), signal_a);
        find_wire_signal(&input, &mut wire_map, "a")
    }
}

fn find_wire_signal(instructions: &Vec<Instruction>, wire_map: &mut HashMap<String, u16>, wire: &str) -> u16 {
    let instruction_for_wire = instructions.iter().find(|i| { i.wire_output == *wire}).unwrap();
    let wire_output = &instruction_for_wire.wire_output;
    match &instruction_for_wire.operation {
        Operation::ASSIGN(value) => {
            if !wire_map.contains_key(wire_output) {
                wire_map.insert(wire_output.to_string(), *value);
            }
        }
        Operation::ASSIGNREF(input) => {
            let output;
            if wire_map.contains_key(input) {
                output = wire_map[input];
            } else {   
                output = find_wire_signal(&instructions, wire_map, input);
            }
            wire_map.insert(wire_output.to_string(), output);
        }
        Operation::AND(input1, input2) => {
            let output1;
            let output2;
            if wire_map.contains_key(input1) && wire_map.contains_key(input2) {
                output1 = wire_map[input1];
                output2 = wire_map[input2];
            } else {
                output1 = if input1 == "1" {1} else{find_wire_signal(&instructions, wire_map, input1)};
                output2 = find_wire_signal(&instructions, wire_map, input2);
            }
            wire_map.insert(wire_output.to_string(), output1 & output2);
        }
        Operation::OR(input1, input2) => {
            let output1;
            let output2;
            if wire_map.contains_key(input1) && wire_map.contains_key(input2) {
                output1 = wire_map[input1];
                output2 = wire_map[input2];
            } else {
                output1 = find_wire_signal(&instructions, wire_map, input1);
                output2 = find_wire_signal(&instructions, wire_map, input2);
            }
            wire_map.insert(wire_output.to_string(), output1 | output2);
        }
        Operation::LSHIFT(input1, input2) => {
            let output1;
            if wire_map.contains_key(input1) {
                output1 = wire_map[input1];
            } else {
                output1 = find_wire_signal(&instructions, wire_map, input1);
            }
            wire_map.insert(wire_output.to_string(), output1 << input2);
        }
        Operation::RSHIFT(input1, input2) => {
            let output1;
            if wire_map.contains_key(input1) {
                output1 = wire_map[input1];
            } else {
                output1 = find_wire_signal(&instructions, wire_map, input1);
            }
            wire_map.insert(wire_output.to_string(), output1 >> input2);
        }
        Operation::NOT(input) => {
            let output;
            if wire_map.contains_key(input) {
                output = wire_map[input];
            } else {
                output = find_wire_signal(&instructions, wire_map, input);
            }
            wire_map.insert(wire_output.to_string(), !output);
        }
    };

    wire_map[wire_output]
}

#[cfg(test)]
mod tests {
    use crate::y2015::day07::*;

    #[test]
    fn test_find_wire_signal() {
        let instructions = vec![
            Instruction::new("x AND y -> d"),
            Instruction::new("x OR y -> e"),
            Instruction::new("x LSHIFT 2 -> f"),
            Instruction::new("y RSHIFT 2 -> g"),
            Instruction::new("NOT x -> h"),
            Instruction::new("NOT y -> i"),
            Instruction::new("123 -> x"),
            Instruction::new("456 -> y"),
        ];

        let mut wire_map: HashMap<String, u16> = HashMap::new();

        assert_eq!(find_wire_signal(&instructions, &mut wire_map, "d"), 72);
        assert_eq!(find_wire_signal(&instructions, &mut wire_map, "e"), 507);
        assert_eq!(find_wire_signal(&instructions, &mut wire_map, "f"), 492);
        assert_eq!(find_wire_signal(&instructions, &mut wire_map, "g"), 114);
        assert_eq!(find_wire_signal(&instructions, &mut wire_map, "h"), 65412);
        assert_eq!(find_wire_signal(&instructions, &mut wire_map, "i"), 65079);
        assert_eq!(find_wire_signal(&instructions, &mut wire_map, "x"), 123);
        assert_eq!(find_wire_signal(&instructions, &mut wire_map, "y"), 456);
    }
}
