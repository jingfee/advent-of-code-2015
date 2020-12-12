use crate::solver::Solver;
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<usize>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> Vec<usize> {
        let buf_reader = BufReader::new(file);
        buf_reader
            .lines()
            .map(|l| l.unwrap().parse::<usize>().unwrap())
            .collect()
    }

    fn solve_part_one(&self, input: &Vec<usize>) -> usize {
        let mut mult = 1;
        expense_entries(&input, 2)
            .iter()
            .for_each(|e| mult = mult * *e);
        mult
    }

    fn solve_part_two(&self, input: &Vec<usize>) -> usize {
        let mut mult = 1;
        expense_entries(&input, 3)
            .iter()
            .for_each(|e| mult = mult * *e);
        mult
    }
}

fn expense_entries(input: &Vec<usize>, number_combinations: usize) -> Vec<&usize> {
    let combinations = input.iter().combinations(number_combinations);
    for combination in combinations {
        let mut sum = 0;
        for entry in &combination {
            sum = sum + *entry;
        }
        if sum == 2020 {
            return combination;
        }
    }

    panic!("No combination found!");
}

#[cfg(test)]
mod tests {
    use crate::y2020::day01::*;

    #[test]
    fn test_expense_entries() {
        let ex = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(expense_entries(&ex, 2), vec![&1721, &299]);
        assert_eq!(expense_entries(&ex, 3), vec![&979, &366, &675]);
    }
}
