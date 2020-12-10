use crate::solver::Solver;
use std::io::prelude::*;
use std::io::BufReader;
use std::{collections::HashMap, fs::File};

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

    fn solve_part_one(&self, input: &mut Vec<usize>) -> usize {
        let differences = find_differences(input);
        differences.0 * differences.1
    }

    fn solve_part_two(&self, input: &mut Vec<usize>) -> usize {
        find_valid_combinations(input)
    }
}

fn find_differences(adapters: &mut Vec<usize>) -> (usize, usize) {
    adapters.sort();
    adapters.push(adapters[adapters.len() - 1] + 3);
    let mut differences = (0, 0);
    let mut jolt = 0;

    for adapter in adapters {
        let diff = *adapter - jolt;

        if diff == 1 {
            differences = (differences.0 + 1, differences.1);
        } else if diff == 3 {
            differences = (differences.0, differences.1 + 1);
        }

        jolt = *adapter;
    }

    differences
}

fn find_valid_combinations(adapters: &mut Vec<usize>) -> usize {
    adapters.sort();
    adapters.insert(0, 0);
    let built_in_adapter = adapters[adapters.len() - 1] + 3;
    adapters.push(built_in_adapter);
    let mut paths = HashMap::new();
    paths.insert(0, 1);

    for adapter in adapters.iter().skip(1) {
        paths.insert(
            *adapter,
            adapters
                .iter()
                .filter(|a| **a < *adapter && **a + 3 >= *adapter)
                .map(|a| paths[a])
                .sum(),
        );
    }

    paths[&built_in_adapter]
}

#[cfg(test)]
mod tests {
    use crate::y2020::day10::*;

    #[test]
    fn test_find_differences() {
        let mut ex1 = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let mut ex2 = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        assert_eq!(find_differences(&mut ex1), (7, 5));
        assert_eq!(find_differences(&mut ex2), (22, 10));
    }

    #[test]
    fn test_find_valid_combinations() {
        let mut ex1 = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let mut ex2 = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        assert_eq!(find_valid_combinations(&mut ex1), 8);
        assert_eq!(find_valid_combinations(&mut ex2), 19208);
    }
}
