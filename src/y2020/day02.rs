use crate::solver::Solver;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct PasswordValidator {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl PasswordValidator {
    fn new(min: usize, max: usize, letter: char, password: String) -> PasswordValidator {
        PasswordValidator {
            min: min,
            max: max,
            letter: letter,
            password: password,
        }
    }

    fn validate(&self) -> bool {
        let num_chars = self.password.chars().filter(|p| p == &self.letter).count();
        num_chars >= self.min && num_chars <= self.max
    }

    fn validate_new_rules(&self) -> bool {
        (self.password.chars().nth(self.min - 1).unwrap() == self.letter)
            ^ (self.password.chars().nth(self.max - 1).unwrap() == self.letter)
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<PasswordValidator>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> Vec<PasswordValidator> {
        let buf_reader = BufReader::new(file);
        buf_reader
            .lines()
            .map(|l| {
                let line = l.unwrap();
                let row_split: Vec<&str> = line.split(' ').collect();
                let range_split: Vec<usize> = row_split[0]
                    .split('-')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
                let letter = row_split[1].chars().nth(0).unwrap();
                PasswordValidator::new(
                    range_split[0],
                    range_split[1],
                    letter,
                    row_split[2].to_string(),
                )
            })
            .collect()
    }

    fn solve_part_one(&self, input: &Vec<PasswordValidator>) -> usize {
        input.iter().filter(|i| i.validate()).count()
    }

    fn solve_part_two(&self, input: &Vec<PasswordValidator>) -> usize {
        input.iter().filter(|i| i.validate_new_rules()).count()
    }
}

#[cfg(test)]
mod tests {
    use crate::y2020::day02::*;

    #[test]
    fn test_password_validate() {
        let ex1 = PasswordValidator::new(1, 3, 'a', "abcde".to_string());
        let ex2 = PasswordValidator::new(1, 3, 'b', "cdefg".to_string());
        let ex3 = PasswordValidator::new(2, 9, 'c', "ccccccccc".to_string());

        assert_eq!(ex1.validate(), true);
        assert_eq!(ex2.validate(), false);
        assert_eq!(ex3.validate(), true);
    }

    #[test]
    fn test_password_validate_new_rules() {
        let ex1 = PasswordValidator::new(1, 3, 'a', "abcde".to_string());
        let ex2 = PasswordValidator::new(1, 3, 'b', "cdefg".to_string());
        let ex3 = PasswordValidator::new(2, 9, 'c', "ccccccccc".to_string());

        assert_eq!(ex1.validate_new_rules(), true);
        assert_eq!(ex2.validate_new_rules(), false);
        assert_eq!(ex3.validate_new_rules(), false);
    }
}
