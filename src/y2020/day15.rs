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
        let line = buf_reader.lines().nth(0).unwrap().unwrap();
        line.split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect()
    }

    fn solve_part_one(&self, input: &Vec<usize>) -> usize {
        play_game(input, 2020)
    }

    fn solve_part_two(&self, input: &Vec<usize>) -> usize {
        play_game(input, 30000000)
    }
}

fn play_game(start_numbers: &Vec<usize>, end_turn: usize) -> usize {
    let mut turn = 1;
    let mut memory_map = HashMap::new();
    let mut next_number = 0;

    for start_number in start_numbers {
        next_number = say_number(&mut memory_map, start_number, turn);
        turn = turn + 1;
    }

    loop {
        next_number = say_number(&mut memory_map, &next_number, turn);

        turn = turn + 1;
        if turn == end_turn {
            return next_number;
        }
    }
}

fn say_number(memory_map: &mut HashMap<usize, usize>, last_number: &usize, turn: usize) -> usize {
    let next_number: usize;
    if memory_map.contains_key(&last_number) {
        next_number = turn - memory_map[&last_number];
    } else {
        next_number = 0;
    }

    memory_map.insert(*last_number, turn);

    next_number
}

#[cfg(test)]
mod tests {
    use crate::y2020::day15::*;

    #[test]
    fn test_play_game() {
        let start_number_ex1 = vec![0, 3, 6];
        let start_number_ex2 = vec![1, 3, 2];
        let start_number_ex3 = vec![2, 1, 3];
        let start_number_ex4 = vec![1, 2, 3];
        let start_number_ex5 = vec![2, 3, 1];
        let start_number_ex6 = vec![3, 2, 1];
        let start_number_ex7 = vec![3, 1, 2];

        assert_eq!(play_game(&start_number_ex1, 2020), 436);
        assert_eq!(play_game(&start_number_ex2, 2020), 1);
        assert_eq!(play_game(&start_number_ex3, 2020), 10);
        assert_eq!(play_game(&start_number_ex4, 2020), 27);
        assert_eq!(play_game(&start_number_ex5, 2020), 78);
        assert_eq!(play_game(&start_number_ex6, 2020), 438);
        assert_eq!(play_game(&start_number_ex7, 2020), 1836);

        assert_eq!(play_game(&start_number_ex1, 30000000), 175594);
        assert_eq!(play_game(&start_number_ex2, 30000000), 2578);
        assert_eq!(play_game(&start_number_ex3, 30000000), 3544142);
        assert_eq!(play_game(&start_number_ex4, 30000000), 261214);
        assert_eq!(play_game(&start_number_ex5, 30000000), 6895259);
        assert_eq!(play_game(&start_number_ex6, 30000000), 18);
        assert_eq!(play_game(&start_number_ex7, 30000000), 362);
    }
}
