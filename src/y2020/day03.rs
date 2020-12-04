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

    fn solve_part_one(&self, input: &mut Vec<String>) -> usize {
        traverse_map(input, 3, 1)
    }

    fn solve_part_two(&self, input: &mut Vec<String>) -> usize {
        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let mut mult = 1;

        for slope in slopes.iter() {
            mult = mult * traverse_map(input, slope.0, slope.1);
        }

        mult
    }
}

fn traverse_map(input: &Vec<String>, right: usize, down: usize) -> usize {
    let mut trees = 0;
    let mut x_index = 0;
    let mut y_index = 0;

    loop {
        let line = &input[y_index];

        if line.chars().nth(x_index).unwrap() == '#' {
            trees = trees + 1;
        }
        x_index = (x_index + right) % line.len();

        y_index = y_index + down;
        if y_index >= input.len() {
            break;
        }
    }

    trees
}

#[cfg(test)]
mod tests {
    use crate::y2020::day03::*;

    #[test]
    fn test_traverse_map() {
        let ex = vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ];

        assert_eq!(traverse_map(&ex, 1, 1), 2);
        assert_eq!(traverse_map(&ex, 3, 1), 7);
        assert_eq!(traverse_map(&ex, 5, 1), 3);
        assert_eq!(traverse_map(&ex, 7, 1), 4);
        assert_eq!(traverse_map(&ex, 1, 2), 2);
    }
}
