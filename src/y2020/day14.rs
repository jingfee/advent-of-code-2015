use crate::solver::Solver;
use itertools::Itertools;
use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::{collections::HashMap, fs::File};

pub struct U36 {
    value: [u8; 36],
}

impl U36 {
    fn new(dec: u64) -> U36 {
        let mut bin = [0; 36];
        let mut curr_dec = dec;
        for i in 0..36 {
            let exp = (2 as u64).pow(35 - i as u32);
            if curr_dec < exp {
                continue;
            }

            bin[i] = 1;
            curr_dec = curr_dec - exp;
        }

        U36 { value: bin }
    }

    fn to_dec(&self) -> u64 {
        let mut val = 0;
        for i in 0..36 {
            val = val + self.value[i] as u64 * (2 as u64).pow(35 - i as u32);
        }

        val
    }

    fn mask(&self, mask: String) -> U36 {
        let mut masked_bin = self.value.clone();

        for (i, c) in mask.chars().enumerate() {
            if c == '1' {
                masked_bin[i] = 1;
            } else if c == '0' {
                masked_bin[i] = 0;
            }
        }

        U36 { value: masked_bin }
    }

    fn mask_ver2(&self, mask: String) -> Vec<u64> {
        let number_of_floats = mask.chars().filter(|c| *c == 'X').count();
        let mut masked_bins = Vec::new();
        let mut possible_values = Vec::new();
        for _i in 0..number_of_floats {
            possible_values.push(0 as u8);
            possible_values.push(1 as u8);
        }
        let combinations = possible_values
            .iter()
            .combinations(number_of_floats)
            .unique();

        for combination in combinations {
            let mut masked_bin = self.value.clone();
            let mut float_count: usize = 0;
            for (i, c) in mask.chars().enumerate() {
                if c == '1' {
                    masked_bin[i] = 1;
                } else if c == 'X' {
                    masked_bin[i] = *combination[float_count] as u8;
                    float_count = float_count + 1;
                }
            }
            masked_bins.push(U36 { value: masked_bin }.to_dec());
        }

        masked_bins
    }
}

pub struct Program {
    mask: String,
    instructions: Vec<(U36, U36)>,
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Program>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(&self, file: File) -> Vec<Program> {
        let buf_reader = BufReader::new(file);
        let mut programs = Vec::new();
        let mut lines = buf_reader.lines();
        let mut line = lines.next().unwrap().unwrap();
        let mut read_all = false;
        loop {
            let mask = line.split('=').collect::<Vec<&str>>()[1];
            let mut program = Program {
                mask: mask.trim().to_string(),
                instructions: Vec::new(),
            };

            loop {
                let inst = lines.next();
                match inst {
                    Some(l) => {
                        let line_string = l.unwrap();
                        if line_string.starts_with("mask") {
                            line = line_string;
                            break;
                        }

                        let re = Regex::new(r"^mem\[([0-9]+)\] = ([0-9]+)$").unwrap();
                        let captures = re.captures(&line_string).unwrap();
                        let mem_dec = captures[1].parse::<u64>().unwrap();
                        let val_dec = captures[2].parse::<u64>().unwrap();
                        program
                            .instructions
                            .push((U36::new(mem_dec), U36::new(val_dec)));
                    }
                    None => {
                        read_all = true;
                        break;
                    }
                }
            }
            programs.push(program);

            if read_all {
                break;
            }
        }
        programs
    }

    fn solve_part_one(&self, input: &Vec<Program>) -> u64 {
        execute_program(input)
    }

    fn solve_part_two(&self, input: &Vec<Program>) -> u64 {
        execute_program_ver2(input)
    }
}

fn execute_program(programs: &Vec<Program>) -> u64 {
    let mut memory_used = HashMap::new();

    for program in programs {
        for instruction in &program.instructions {
            let masked_value = instruction.1.mask(program.mask.to_string());
            memory_used.insert(instruction.0.to_dec(), masked_value.to_dec());
        }
    }

    memory_used.values().sum()
}

fn execute_program_ver2(programs: &Vec<Program>) -> u64 {
    let mut memory_used = HashMap::new();

    for program in programs {
        for instruction in &program.instructions {
            let addresses = instruction.0.mask_ver2(program.mask.to_string());

            for address in addresses {
                memory_used.insert(address, instruction.1.to_dec());
            }
        }
    }

    memory_used.values().sum()
}

#[cfg(test)]
mod tests {
    use crate::y2020::day14::*;

    #[test]
    fn test_execute_program() {
        let ex = vec![Program {
            mask: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            instructions: vec![
                (U36::new(8), U36::new(11)),
                (U36::new(7), U36::new(101)),
                (U36::new(8), U36::new(0)),
            ],
        }];

        assert_eq!(execute_program(&ex), 165);
    }

    #[test]
    fn test_execute_program_ver2() {
        let ex = vec![
            Program {
                mask: "000000000000000000000000000000X1001X".to_string(),
                instructions: vec![(U36::new(42), U36::new(100))],
            },
            Program {
                mask: "00000000000000000000000000000000X0XX".to_string(),
                instructions: vec![(U36::new(26), U36::new(1))],
            },
        ];

        assert_eq!(execute_program_ver2(&ex), 208);
    }
}
