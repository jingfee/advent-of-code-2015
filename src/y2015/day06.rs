use crate::solver::Solver;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

enum Operation {
    Off,
    On,
    Toggle,
}

pub struct Instruction {
    operation: Operation,
    range_from: (usize, usize),
    range_to: (usize, usize),
}

impl Instruction {
    fn new(from: &str, to: &str, operation: Operation) -> Instruction {
        let from_split: Vec<&str> = from.split(',').collect();
        let to_split: Vec<&str> = to.split(',').collect();
        Instruction {
            operation: operation,
            range_from: (
                from_split[0].parse::<usize>().unwrap(),
                from_split[1].parse::<usize>().unwrap(),
            ),
            range_to: (
                to_split[0].parse::<usize>().unwrap(),
                to_split[1].parse::<usize>().unwrap(),
            ),
        }
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Instruction>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> Vec<Instruction> {
        let buf_reader = BufReader::new(file);
        buf_reader
            .lines()
            .map(|l| {
                let l = l.unwrap();
                let row_split: Vec<&str> = l.split(' ').collect();
                if row_split[0] == "toggle" {
                    Instruction::new(row_split[1], row_split[3], Operation::Toggle)
                } else if row_split[1] == "on" {
                    Instruction::new(row_split[2], row_split[4], Operation::On)
                } else {
                    Instruction::new(row_split[2], row_split[4], Operation::Off)
                }
            })
            .collect()
    }

    fn solve_part_one(&self, input: &mut Vec<Instruction>) -> usize {
        let mut light_grid = vec![vec![false; 1000]; 1000];
        input.iter().for_each(|instruction| {
            apply_instruction(&instruction, &mut light_grid);
        });
        light_grid
            .iter()
            .map(|row| row.iter().filter(|l| **l).count())
            .sum()
    }

    fn solve_part_two(&self, input: &mut Vec<Instruction>) -> usize {
        let mut light_grid = vec![vec![0; 1000]; 1000];
        input.iter().for_each(|instruction| {
            apply_instruction_brightness(&instruction, &mut light_grid);
        });
        light_grid.iter().flat_map(|x: &Vec<_>| x.iter()).sum()
    }
}

fn apply_instruction(instruction: &Instruction, light_grid: &mut Vec<Vec<bool>>) -> usize {
    let mut lights = 0;
    for i in instruction.range_from.0..(instruction.range_to.0 + 1) {
        for j in instruction.range_from.1..(instruction.range_to.1 + 1) {
            match instruction.operation {
                Operation::Off => light_grid[i][j] = false,
                Operation::On => light_grid[i][j] = true,
                Operation::Toggle => light_grid[i][j] = !light_grid[i][j],
            }
            lights = lights + 1;
        }
    }

    lights
}

fn apply_instruction_brightness(
    instruction: &Instruction,
    light_grid: &mut Vec<Vec<usize>>,
) -> isize {
    let mut brightness = 0;
    for i in instruction.range_from.0..(instruction.range_to.0 + 1) {
        for j in instruction.range_from.1..(instruction.range_to.1 + 1) {
            match instruction.operation {
                Operation::Off => {
                    light_grid[i][j] = if light_grid[i][j] == 0 {
                        0
                    } else {
                        light_grid[i][j] - 1
                    };
                    brightness = brightness - 1;
                }
                Operation::On => {
                    light_grid[i][j] = light_grid[i][j] + 1;
                    brightness = brightness + 1
                }
                Operation::Toggle => {
                    light_grid[i][j] = light_grid[i][j] + 2;
                    brightness = brightness + 2
                }
            }
        }
    }
    brightness
}

#[cfg(test)]
mod tests {
    use crate::y2015::day06::*;

    #[test]
    fn test_apply_instruction() {
        let instruction1 = Instruction::new("0,0", "999,999", Operation::On);
        let instruction2 = Instruction::new("0,0", "999,0", Operation::Toggle);
        let instruction3 = Instruction::new("499,499", "500,500", Operation::Off);

        let mut light_grid = vec![vec![false; 1000]; 1000];

        assert_eq!(apply_instruction(&instruction1, &mut light_grid), 1000000);
        assert_eq!(apply_instruction(&instruction2, &mut light_grid), 1000);
        assert_eq!(apply_instruction(&instruction3, &mut light_grid), 4);
    }

    #[test]
    fn test_apply_instruction_brightness() {
        let instruction1 = Instruction::new("0,0", "0,0", Operation::On);
        let instruction2 = Instruction::new("0,0", "999,999", Operation::Toggle);

        let mut light_grid = vec![vec![0; 1000]; 1000];

        assert_eq!(
            apply_instruction_brightness(&instruction1, &mut light_grid),
            1
        );
        assert_eq!(
            apply_instruction_brightness(&instruction2, &mut light_grid),
            2000000
        );
    }
}
