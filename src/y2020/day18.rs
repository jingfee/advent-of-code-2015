use crate::solver::Solver;
use itertools::Itertools;
use std::io::prelude::*;
use std::io::BufReader;
use std::{fs::File, str::Chars};

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
        input.iter().map(|l| calc_string(&mut l.chars())).sum()
    }

    fn solve_part_two(&self, input: &Vec<String>) -> usize {
        input
            .iter()
            .map(|l| calc_string_with_precedence(&l.chars().collect::<Vec<char>>(), &mut 0, false))
            .sum()
    }
}

fn calc_string(input: &mut Chars) -> usize {
    let mut val = 0;
    let mut current_operator = '+';
    loop {
        let next = input.next();

        match next {
            Some(c) => {
                if c == ')' {
                    break;
                } else if c == '(' {
                    let parentheses_value = calc_string(input);
                    val = if current_operator == '+' {
                        val + parentheses_value
                    } else {
                        val * parentheses_value
                    };
                } else if c == '+' {
                    current_operator = '+';
                } else if c == '*' {
                    current_operator = '*';
                } else if c == ' ' {
                    continue;
                } else {
                    let parsed_value = c.to_string().parse::<usize>().unwrap();
                    val = if current_operator == '+' {
                        val + parsed_value
                    } else {
                        val * parsed_value
                    };
                }
            }
            None => break,
        }
    }
    val
}

fn calc_string_with_precedence(input: &Vec<char>, index: &mut usize, find_addition: bool) -> usize {
    let mut val = 0;
    let mut current_operator = '+';
    loop {
        if *index == input.len() {
            break;
        }
        let c = input[*index];
        *index = *index + 1;

        if c == ')' {
            if find_addition {
                *index = *index - 1;
            }
            break;
        } else if c == '(' {
            let parentheses_value = calc_string_with_precedence(input, index, false);
            val = if current_operator == '+' {
                val + parentheses_value
            } else {
                val * parentheses_value
            };
        } else if c == '+' {
            current_operator = '+';
        } else if c == '*' {
            if find_addition {
                *index = *index - 1;
                break;
            }
            val = val * calc_string_with_precedence(input, index, true);
        } else if c == ' ' {
            continue;
        } else {
            let parsed_value = c.to_string().parse::<usize>().unwrap();
            val = val + parsed_value;
        }
    }
    val
}

#[cfg(test)]
mod tests {
    use crate::y2020::day18::*;

    #[test]
    fn test_calc_string() {
        let ex1 = "1 + 2 * 3 + 4 * 5 + 6".to_string();
        let ex2 = "1 + (2 * 3) + (4 * (5 + 6))".to_string();
        let ex3 = "2 * 3 + (4 * 5)".to_string();
        let ex4 = "5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string();
        let ex5 = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string();
        let ex6 = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string();

        assert_eq!(calc_string(&mut ex1.chars()), 71);
        assert_eq!(calc_string(&mut ex2.chars()), 51);
        assert_eq!(calc_string(&mut ex3.chars()), 26);
        assert_eq!(calc_string(&mut ex4.chars()), 437);
        assert_eq!(calc_string(&mut ex5.chars()), 12240);
        assert_eq!(calc_string(&mut ex6.chars()), 13632);
    }

    #[test]
    fn test_calc_string_with_precedence() {
        let ex1 = "1 + 2 * 3 + 4 * 5 + 6".to_string();
        let ex2 = "1 + (2 * 3) + (4 * (5 + 6))".to_string();
        let ex3 = "2 * 3 + (4 * 5)".to_string();
        let ex4 = "5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string();
        let ex5 = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string();
        let ex6 = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string();

        assert_eq!(
            calc_string_with_precedence(&mut ex1.chars().collect::<Vec<char>>(), &mut 0, false),
            231
        );
        assert_eq!(
            calc_string_with_precedence(&mut ex2.chars().collect::<Vec<char>>(), &mut 0, false),
            51
        );
        assert_eq!(
            calc_string_with_precedence(&mut ex3.chars().collect::<Vec<char>>(), &mut 0, false),
            46
        );
        assert_eq!(
            calc_string_with_precedence(&mut ex4.chars().collect::<Vec<char>>(), &mut 0, false),
            1445
        );
        assert_eq!(
            calc_string_with_precedence(&mut ex5.chars().collect::<Vec<char>>(), &mut 0, false),
            669060
        );
        assert_eq!(
            calc_string_with_precedence(&mut ex6.chars().collect::<Vec<char>>(), &mut 0, false),
            23340
        );
    }
}
