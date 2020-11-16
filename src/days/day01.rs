use crate::solver::Solver;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<char>;
    type Output1 = i32;
    type Output2 = i32;

    fn parse_input(&self, file: File) -> Vec<char> {
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        return contents.chars().collect();
    }

    fn solve_part_one(&self, input: &Vec<char>) -> i32 {
        let mut level = 0;
        for instruction in input.iter() {
            if *instruction == '(' {
                level = level + 1;
            } else if *instruction == ')' {
                level = level - 1;
            }
        }
        return level;
    }

    fn solve_part_two(&self, input: &Vec<char>) -> i32 {
        let mut level = 0;
        for (index, instruction) in input.iter().enumerate() {
            if *instruction == '(' {
                level = level + 1;
            } else if *instruction == ')' {
                level = level - 1;
            }

            if level == -1 {
                return index as i32 + 1;
            }
        }

        -1
    }
}
