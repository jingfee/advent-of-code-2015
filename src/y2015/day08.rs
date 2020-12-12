use crate::solver::Solver;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<String>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> Vec<String> {
        let buf_reader = BufReader::new(file);
        buf_reader.lines().map(|l| l.unwrap()).collect()
    }

    fn solve_part_one(&self, input: &Vec<String>) -> usize {
        input
            .iter()
            .map(|i| code_length(&i) - string_length(&i))
            .sum()
    }

    fn solve_part_two(&self, input: &Vec<String>) -> usize {
        input
            .iter()
            .map(|i| encoded_length(&i) - code_length(&i))
            .sum()
    }
}

fn string_length(line: &String) -> usize {
    let mut length = code_length(&line);
    length = length - 2; // starting and trailing double-quotes

    let mut index = 1;
    while index < line.chars().count() - 1 {
        let c = line.chars().nth(index).unwrap();
        if c == '\\' {
            if line.chars().nth(index + 1).unwrap() == 'x' {
                length = length - 3;
                index = index + 4;
            } else {
                length = length - 1;
                index = index + 2;
            }
        } else {
            index = index + 1;
        }
    }

    length
}

fn code_length(line: &String) -> usize {
    line.chars().count()
}

fn encoded_length(line: &String) -> usize {
    let mut length = code_length(&line);
    length = length + 4; // starting and trailing double-quotes

    let mut index = 1;
    while index < line.chars().count() - 1 {
        let c = line.chars().nth(index).unwrap();
        if c == '\\' {
            if line.chars().nth(index + 1).unwrap() == 'x' {
                length = length + 1;
                index = index + 4;
            } else {
                length = length + 2;
                index = index + 2;
            }
        } else {
            index = index + 1;
        }
    }

    length
}

#[cfg(test)]
mod tests {
    use crate::y2015::day08::*;

    #[test]
    fn test_string_length() {
        let ex1 = "\"\"";
        let ex2 = "\"abc\"";
        let ex3 = "\"aaa\\\"aaa\"";
        let ex4 = "\"\\x27\"";

        assert_eq!(string_length(&ex1.to_string()), 0);
        assert_eq!(string_length(&ex2.to_string()), 3);
        assert_eq!(string_length(&ex3.to_string()), 7);
        assert_eq!(string_length(&ex4.to_string()), 1);
    }

    #[test]
    fn test_code_length() {
        let ex1 = "\"\"";
        let ex2 = "\"abc\"";
        let ex3 = "\"aaa\\\"aaa\"";
        let ex4 = "\"\\x27\"";

        assert_eq!(code_length(&ex1.to_string()), 2);
        assert_eq!(code_length(&ex2.to_string()), 5);
        assert_eq!(code_length(&ex3.to_string()), 10);
        assert_eq!(code_length(&ex4.to_string()), 6);
    }

    #[test]
    fn test_encoded_length() {
        let ex1 = "\"\"";
        let ex2 = "\"abc\"";
        let ex3 = "\"aaa\\\"aaa\"";
        let ex4 = "\"\\x27\"";

        assert_eq!(encoded_length(&ex1.to_string()), 6);
        assert_eq!(encoded_length(&ex2.to_string()), 9);
        assert_eq!(encoded_length(&ex3.to_string()), 16);
        assert_eq!(encoded_length(&ex4.to_string()), 11);
    }
}
