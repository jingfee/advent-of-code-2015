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
        get_floor(&input)
    }

    fn solve_part_two(&self, input: &Vec<char>) -> i32 {
        get_basement_position(input)
    }
}

fn get_floor(input: &Vec<char>) -> i32 {
    let mut level = 0;
    for instruction in input.iter() {
        if *instruction == '(' {
            level = level + 1;
        } else if *instruction == ')' {
            level = level - 1;
        }
    }
    level
}

fn get_basement_position(input: &Vec<char>) -> i32 {
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

#[cfg(test)]
mod tests {
    use crate::days::day01::*;

    #[test]
    fn test_get_floor() {
        let ex1: Vec<char> = "(())".chars().collect();
        let ex2: Vec<char> = "(((".chars().collect();
        let ex3: Vec<char> = "(()(()(".chars().collect();
        let ex4: Vec<char> = "))(((((".chars().collect();
        let ex5: Vec<char> = "())".chars().collect();
        let ex6: Vec<char> = "))(".chars().collect();
        let ex7: Vec<char> = ")))".chars().collect();
        let ex8: Vec<char> = ")())())".chars().collect();

        assert_eq!(get_floor(&ex1), 0);
        assert_eq!(get_floor(&ex2), 3);
        assert_eq!(get_floor(&ex3), 3);
        assert_eq!(get_floor(&ex4), 3);
        assert_eq!(get_floor(&ex5), -1);
        assert_eq!(get_floor(&ex6), -1);
        assert_eq!(get_floor(&ex7), -3);
        assert_eq!(get_floor(&ex8), -3);
    }

    #[test]
    fn test_get_basement_position() {
        let ex1: Vec<char> = ")".chars().collect();
        let ex2: Vec<char> = "()())".chars().collect();

        assert_eq!(get_basement_position(&ex1), 1);
        assert_eq!(get_basement_position(&ex2), 5);
    }
}
