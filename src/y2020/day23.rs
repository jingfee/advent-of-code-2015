use crate::solver::Solver;
use std::io::prelude::*;
use std::io::BufReader;
use std::{collections::HashMap, fs::File};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<usize>;
    type Output1 = String;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> Vec<usize> {
        let mut buf_reader = BufReader::new(file);
        let mut line = String::new();
        buf_reader.read_to_string(&mut line).unwrap();
        line.chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect()
    }

    fn solve_part_one(&self, input: &Vec<usize>) -> String {
        let mut cups = create_map_part_1(&input);
        play_game(&mut cups, input[0], 100);

        let mut result = String::new();
        let mut i = 1;
        for _j in 0..8 {
            result.push_str(&cups[&i].to_string());
            i = cups[&i];
        }

        result
    }

    fn solve_part_two(&self, input: &Vec<usize>) -> usize {
        let mut cups = create_map_part_2(&input);
        play_game(&mut cups, input[0], 10000000);

        cups[&1] * cups[&cups[&1]]
    }
}

fn create_map_part_1(input: &Vec<usize>) -> HashMap<usize, usize> {
    let mut cups = HashMap::new();
    for (i, c) in input.iter().enumerate() {
        cups.insert(*c, input[(i + 1) % input.len()]);
    }
    cups
}

fn create_map_part_2(input: &Vec<usize>) -> HashMap<usize, usize> {
    let mut cups = HashMap::new();
    for i in 1..1000000 {
        if i == 9 {
            cups.insert(input[i - 1], 10);
        } else if i < 9 {
            cups.insert(input[i - 1], input[i]);
        } else {
            cups.insert(i, i + 1);
        }
    }
    cups.insert(1000000, input[0]);
    cups
}

fn play_game(cups: &mut HashMap<usize, usize>, first_cup: usize, rounds: usize) {
    let mut current_cup = first_cup;

    for _i in 0..rounds {
        let mut moved_cups = Vec::new();
        let mut c = current_cup;
        for _j in 0..3 {
            c = cups[&c];
            moved_cups.push(c);
        }

        cups.insert(current_cup, cups[&c]);

        let mut destination = current_cup;
        loop {
            destination = destination - 1;
            if destination == 0 {
                destination = cups.len();
            }
            if moved_cups.contains(&destination) {
                continue;
            }
            break;
        }

        let next_after_destination = cups[&destination];
        cups.insert(destination, moved_cups[0]);
        cups.insert(moved_cups[2], next_after_destination);

        current_cup = cups[&current_cup];
    }
}

#[cfg(test)]
mod tests {
    use crate::y2020::day23::*;

    #[test]
    fn test_play_game() {
        let ex1 = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let mut map1 = create_map_part_1(&ex1);
        play_game(&mut map1, 3, 100);

        let mut result = String::new();
        let mut i = 1;
        for _j in 0..8 {
            result.push_str(&map1[&i].to_string());
            i = map1[&i];
        }

        let ex2 = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let mut map2 = create_map_part_2(&ex2);
        play_game(&mut map2, 3, 10000000);

        assert_eq!(result, "67384529");
        assert_eq!(map2[&1], 934001);
        assert_eq!(map2[&map2[&1]], 159792);
    }
}
