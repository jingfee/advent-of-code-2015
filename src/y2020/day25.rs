use crate::solver::Solver;
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Problem;

impl Solver for Problem {
    type Input = (usize, usize);
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> (usize, usize) {
        let buf_reader = BufReader::new(file);
        let mut lines = buf_reader.lines();
        (
            lines.nth(0).unwrap().unwrap().parse::<usize>().unwrap(),
            lines.nth(0).unwrap().unwrap().parse::<usize>().unwrap(),
        )
    }

    fn solve_part_one(&self, input: &(usize, usize)) -> usize {
        let loop_size_card = find_loop_size(input.0);
        transform_subject_number(input.1, loop_size_card)
    }

    fn solve_part_two(&self, input: &(usize, usize)) -> usize {
        0
    }
}

fn find_loop_size(public_key: usize) -> usize {
    let mut i = 1;
    let mut value = 1;
    loop {
        value = (value * 7) % 20201227;
        if value == public_key {
            break;
        }

        i = i + 1;
    }
    i
}

fn transform_subject_number(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _i in 0..loop_size {
        value = (value * subject_number) % 20201227;
    }
    value
}

#[cfg(test)]
mod tests {
    use crate::y2020::day25::*;

    #[test]
    fn test_find_loop_size() {
        assert_eq!(find_loop_size(5764801), 8);
        assert_eq!(find_loop_size(17807724), 11);

        assert_eq!(transform_subject_number(17807724, 8), 14897079);
        assert_eq!(transform_subject_number(5764801, 11), 14897079);
    }
}
