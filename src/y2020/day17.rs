use crate::solver::Solver;
use std::io::prelude::*;
use std::io::BufReader;
use std::{collections::HashMap, fs::File};

pub struct Problem;

impl Solver for Problem {
    type Input = HashMap<Vec<isize>, bool>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> HashMap<Vec<isize>, bool> {
        let buf_reader = BufReader::new(file);
        let lines = buf_reader
            .lines()
            .map(|l| l.unwrap())
            .collect::<Vec<String>>();
        parse_lines(lines)
    }

    fn solve_part_one(&self, input: &HashMap<Vec<isize>, bool>) -> usize {
        simulate(input, 6, 3)
    }

    fn solve_part_two(&self, input: &HashMap<Vec<isize>, bool>) -> usize {
        simulate(input, 6, 4)
    }
}

fn parse_lines(lines: Vec<String>) -> HashMap<Vec<isize>, bool> {
    let mut map = HashMap::new();

    for i in 0..lines.len() {
        for (j, c) in lines[i].chars().enumerate() {
            if c == '#' {
                map.insert(vec![j as isize, i as isize], true);
            } else {
                map.insert(vec![j as isize, i as isize], false);
            }
        }
    }

    map
}

fn simulate(
    initial_state: &HashMap<Vec<isize>, bool>,
    rounds_to_simulate: usize,
    dimensions: usize,
) -> usize {
    let mut iterations = 0;
    let mut state = HashMap::new();
    for (k, v) in initial_state {
        let mut coordinates = k.clone();
        for _d in 0..dimensions - 2 {
            coordinates.push(0);
        }

        state.insert(coordinates, *v);
    }

    loop {
        state = if dimensions == 3 {
            run_cycle_3d(&state)
        } else {
            run_cycle_4d(&state)
        };
        iterations = iterations + 1;
        if iterations == rounds_to_simulate {
            break;
        }
    }

    state.values().filter(|v| **v).count()
}

fn run_cycle_3d(state: &HashMap<Vec<isize>, bool>) -> HashMap<Vec<isize>, bool> {
    let min_x = state.keys().map(|k| k[0]).min().unwrap() - 1;
    let max_x = state.keys().map(|k| k[0]).max().unwrap() + 1;
    let min_y = state.keys().map(|k| k[1]).min().unwrap() - 1;
    let max_y = state.keys().map(|k| k[1]).max().unwrap() + 1;
    let min_z = state.keys().map(|k| k[2]).min().unwrap() - 1;
    let max_z = state.keys().map(|k| k[2]).max().unwrap() + 1;

    let mut new_state = HashMap::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                if state.contains_key(&vec![x, y, z]) && state[&vec![x, y, z]] {
                    let active_neighbours = find_neighbours_3d(x, y, z)
                        .iter()
                        .filter(|n| state.contains_key(*n) && state[*n])
                        .count();

                    if active_neighbours == 2 || active_neighbours == 3 {
                        new_state.insert(vec![x, y, z], true);
                    } else {
                        new_state.insert(vec![x, y, z], false);
                    }
                } else {
                    let active_neighbours = find_neighbours_3d(x, y, z)
                        .iter()
                        .filter(|n| state.contains_key(*n) && state[*n])
                        .count();

                    if active_neighbours == 3 {
                        new_state.insert(vec![x, y, z], true);
                    } else {
                        new_state.insert(vec![x, y, z], false);
                    }
                }
            }
        }
    }

    new_state
}

fn find_neighbours_3d(x: isize, y: isize, z: isize) -> Vec<Vec<isize>> {
    let mut neighbours = Vec::new();

    for x_diff in -1..=1 {
        for y_diff in -1..=1 {
            for z_diff in -1..=1 {
                if x_diff == 0 && y_diff == 0 && z_diff == 0 {
                    continue;
                }

                neighbours.push(vec![x + x_diff, y + y_diff, z + z_diff]);
            }
        }
    }

    neighbours
}

fn run_cycle_4d(state: &HashMap<Vec<isize>, bool>) -> HashMap<Vec<isize>, bool> {
    let min_x = state.keys().map(|k| k[0]).min().unwrap() - 1;
    let max_x = state.keys().map(|k| k[0]).max().unwrap() + 1;
    let min_y = state.keys().map(|k| k[1]).min().unwrap() - 1;
    let max_y = state.keys().map(|k| k[1]).max().unwrap() + 1;
    let min_z = state.keys().map(|k| k[2]).min().unwrap() - 1;
    let max_z = state.keys().map(|k| k[2]).max().unwrap() + 1;
    let min_w = state.keys().map(|k| k[3]).min().unwrap() - 1;
    let max_w = state.keys().map(|k| k[3]).max().unwrap() + 1;

    let mut new_state = HashMap::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                for w in min_w..=max_w {
                    if state.contains_key(&vec![x, y, z, w]) && state[&vec![x, y, z, w]] {
                        let active_neighbours = find_neighbours_4d(x, y, z, w)
                            .iter()
                            .filter(|n| state.contains_key(*n) && state[*n])
                            .count();

                        if active_neighbours == 2 || active_neighbours == 3 {
                            new_state.insert(vec![x, y, z, w], true);
                        } else {
                            new_state.insert(vec![x, y, z, w], false);
                        }
                    } else {
                        let active_neighbours = find_neighbours_4d(x, y, z, w)
                            .iter()
                            .filter(|n| state.contains_key(*n) && state[*n])
                            .count();

                        if active_neighbours == 3 {
                            new_state.insert(vec![x, y, z, w], true);
                        } else {
                            new_state.insert(vec![x, y, z, w], false);
                        }
                    }
                }
            }
        }
    }

    new_state
}

fn find_neighbours_4d(x: isize, y: isize, z: isize, w: isize) -> Vec<Vec<isize>> {
    let mut neighbours = Vec::new();

    for x_diff in -1..=1 {
        for y_diff in -1..=1 {
            for z_diff in -1..=1 {
                for w_diff in -1..=1 {
                    if x_diff == 0 && y_diff == 0 && z_diff == 0 && w_diff == 0 {
                        continue;
                    }

                    neighbours.push(vec![x + x_diff, y + y_diff, z + z_diff, w + w_diff]);
                }
            }
        }
    }

    neighbours
}

#[cfg(test)]
mod tests {
    use crate::y2020::day17::*;

    #[test]
    fn test_simulate() {
        let init_state = parse_lines(vec![
            ".#.".to_string(),
            "..#".to_string(),
            "###".to_string(),
        ]);

        assert_eq!(simulate(&init_state, 6, 3), 112);
        assert_eq!(simulate(&init_state, 6, 4), 848);
    }
}
