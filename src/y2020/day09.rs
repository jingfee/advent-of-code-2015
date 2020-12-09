use crate::solver::Solver;
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

    fn solve_part_one(&self, input: &mut Vec<usize>) -> usize {
        find_first_fault_in_xmas(input, 25)
    }

    fn solve_part_two(&self, input: &mut Vec<usize>) -> usize {
        find_weakness(input, find_first_fault_in_xmas(input, 25))
    }
}

fn find_first_fault_in_xmas(input: &Vec<usize>, preamble_length: usize) -> usize {
    let mut first_fault = None;
    let mut index = preamble_length;
    for num in input.iter().skip(preamble_length) {
        let mut valid = false;
        for a in index - preamble_length..index {
            for b in index - preamble_length..index {
                if a == b {
                    continue;
                }

                if input[a] + input[b] == *num {
                    valid = true;
                    break;
                }
            }

            if valid {
                break;
            }
        }

        if !valid {
            first_fault = Some(*num);
            break;
        }

        index = index + 1;
    }

    first_fault.unwrap()
}

fn find_weakness(input: &Vec<usize>, first_fault: usize) -> usize {
    let mut contiguous_set = Vec::new();

    let mut index = 0;
    while index < input.len() {
        let sum: usize = contiguous_set.iter().sum();

        if sum == first_fault {
            break;
        } else if sum > first_fault {
            contiguous_set.remove(0);
        } else {
            contiguous_set.push(input[index]);
            index = index + 1;
        }
    }

    contiguous_set.iter().min().unwrap() + contiguous_set.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::y2020::day09::*;

    #[test]
    fn test_find_first_fault_in_xmas() {
        let ex = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(find_first_fault_in_xmas(&ex, 5), 127);
    }

    #[test]
    fn test_find_weakness() {
        let ex = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(find_weakness(&ex, 127), 62);
    }
}
