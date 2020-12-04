use crate::solver::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<char>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse_input(&self, file: File) -> Vec<char> {
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        return contents.chars().collect();
    }

    fn solve_part_one(&self, input: &mut Vec<char>) -> u32 {
        get_houses(&input)
    }

    fn solve_part_two(&self, input: &mut Vec<char>) -> u32 {
        get_houses_robot(&input)
    }
}

fn get_houses(input: &Vec<char>) -> u32 {
    let mut houses = HashSet::new();
    let mut current_coordinate = (0, 0);
    houses.insert(current_coordinate);
    for instruction in input.iter() {
        if *instruction == '^' {
            current_coordinate = (current_coordinate.0, current_coordinate.1 - 1);
        } else if *instruction == '>' {
            current_coordinate = (current_coordinate.0 + 1, current_coordinate.1);
        } else if *instruction == 'v' {
            current_coordinate = (current_coordinate.0, current_coordinate.1 + 1);
        } else {
            current_coordinate = (current_coordinate.0 - 1, current_coordinate.1);
        }

        houses.insert(current_coordinate);
    }

    houses.len() as u32
}

fn get_houses_robot(input: &Vec<char>) -> u32 {
    let mut houses = HashSet::new();
    let mut coordinates = [(0, 0), (0, 0)];
    let mut pointer = 0;
    houses.insert(coordinates[pointer]);
    for instruction in input.iter() {
        if *instruction == '^' {
            coordinates[pointer] = (coordinates[pointer].0, coordinates[pointer].1 - 1);
        } else if *instruction == '>' {
            coordinates[pointer] = (coordinates[pointer].0 + 1, coordinates[pointer].1);
        } else if *instruction == 'v' {
            coordinates[pointer] = (coordinates[pointer].0, coordinates[pointer].1 + 1);
        } else {
            coordinates[pointer] = (coordinates[pointer].0 - 1, coordinates[pointer].1);
        }

        houses.insert(coordinates[pointer]);

        pointer = if pointer == 0 { 1 } else { 0 };
    }

    houses.len() as u32
}

#[cfg(test)]
mod tests {
    use crate::y2015::day03::*;

    #[test]
    fn test_get_houses() {
        let ex1: Vec<char> = ">".chars().collect();
        let ex2: Vec<char> = "^>v<".chars().collect();
        let ex3: Vec<char> = "^v^v^v^v^v".chars().collect();

        assert_eq!(get_houses(&ex1), 2);
        assert_eq!(get_houses(&ex2), 4);
        assert_eq!(get_houses(&ex3), 2);
    }

    #[test]
    fn test_get_houses_robot() {
        let ex1: Vec<char> = "^v".chars().collect();
        let ex2: Vec<char> = "^>v<".chars().collect();
        let ex3: Vec<char> = "^v^v^v^v^v".chars().collect();

        assert_eq!(get_houses_robot(&ex1), 3);
        assert_eq!(get_houses_robot(&ex2), 3);
        assert_eq!(get_houses_robot(&ex3), 11);
    }
}
