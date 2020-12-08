use crate::solver::Solver;
use std::io::prelude::*;
use std::io::BufReader;
use std::{collections::HashSet, fs::File};

#[derive(Clone)]
pub enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Instruction>;
    type Output1 = isize;
    type Output2 = isize;

    fn parse_input(&self, file: File) -> Vec<Instruction> {
        let buf_reader = BufReader::new(file);
        buf_reader
            .lines()
            .map(|l| get_instruction(&l.unwrap()))
            .collect()
    }

    fn solve_part_one(&self, input: &mut Vec<Instruction>) -> isize {
        find_loop(input).0
    }

    fn solve_part_two(&self, input: &mut Vec<Instruction>) -> isize {
        fix_and_run_instructions(input)
    }
}

fn get_instruction(line: &String) -> Instruction {
    let line_split: Vec<&str> = line.split(' ').collect();
    match line_split[0] {
        "nop" => Instruction::Nop(line_split[1].parse::<isize>().unwrap()),
        "acc" => Instruction::Acc(line_split[1].parse::<isize>().unwrap()),
        "jmp" => Instruction::Jmp(line_split[1].parse::<isize>().unwrap()),
        _ => panic!("Unexpected instruction"),
    }
}

fn find_loop(instructions: &Vec<Instruction>) -> (isize, bool) {
    let mut executed_instructions: HashSet<isize> = HashSet::new();
    let mut pointer = 0;
    let mut accumulator = 0;

    loop {
        if executed_instructions.contains(&pointer) {
            return (accumulator, true);
        }

        if pointer as usize == instructions.len() {
            return (accumulator, false);
        }

        let instruction = &instructions[pointer as usize];
        executed_instructions.insert(pointer);
        pointer = pointer + 1;
        match instruction {
            Instruction::Nop(_n) => continue,
            Instruction::Acc(a) => accumulator = accumulator + a,
            Instruction::Jmp(j) => pointer = pointer + j - 1,
        }
    }
}

fn fix_and_run_instructions(instructions: &Vec<Instruction>) -> isize {
    for (i, instruction) in instructions.iter().enumerate() {
        let mut fixed_instructions = instructions.to_vec();
        match instruction {
            Instruction::Nop(n) => {
                fixed_instructions[i] = Instruction::Jmp(*n);
            }
            Instruction::Acc(_a) => continue,
            Instruction::Jmp(j) => {
                fixed_instructions[i] = Instruction::Nop(*j);
            }
        }

        let result = find_loop(&fixed_instructions);
        if !result.1 {
            return result.0;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use crate::y2020::day08::*;

    #[test]
    fn test_find_loop() {
        let instructions = vec![
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        ]
        .iter()
        .map(|l| get_instruction(l))
        .collect::<Vec<Instruction>>();

        assert_eq!(find_loop(&instructions).0, 5);
    }

    #[test]
    fn test_fix_and_run_instructions() {
        let instructions = vec![
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        ]
        .iter()
        .map(|l| get_instruction(l))
        .collect::<Vec<Instruction>>();

        assert_eq!(fix_and_run_instructions(&instructions), 8);
    }
}
