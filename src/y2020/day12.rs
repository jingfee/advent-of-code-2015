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

    fn solve_part_one(&self, input: &Vec<String>) -> usize {
        navigate(input)
    }

    fn solve_part_two(&self, input: &Vec<String>) -> usize {
        navigate_waypoint(input)
    }
}

fn navigate(instructions: &Vec<String>) -> usize {
    let mut coordinates = (0, 0);
    let mut face_direction = 'E';

    for instruction in instructions {
        let action = instruction.chars().nth(0).unwrap();
        let value = instruction
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<isize>()
            .unwrap();

        if action == 'N' || action == 'S' || action == 'E' || action == 'W' {
            coordinates = travel_direction(action, value, coordinates);
        } else if action == 'F' {
            coordinates = travel_direction(face_direction, value, coordinates)
        } else {
            face_direction = turn(face_direction, value, action);
        }
    }

    coordinates.0.abs() as usize + coordinates.1.abs() as usize
}

fn navigate_waypoint(instructions: &Vec<String>) -> usize {
    let mut coordinates = (0, 0);
    let mut waypoint = (10, 1);

    for instruction in instructions {
        let action = instruction.chars().nth(0).unwrap();
        let value = instruction
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<isize>()
            .unwrap();

        if action == 'N' || action == 'S' || action == 'E' || action == 'W' {
            waypoint = travel_direction(action, value, waypoint);
        } else if action == 'F' {
            for _i in 0..value {
                coordinates = (coordinates.0 + waypoint.0, coordinates.1 + waypoint.1);
            }
        } else {
            waypoint = rotate_waypoint(waypoint, value, action);
        }
    }

    coordinates.0.abs() as usize + coordinates.1.abs() as usize
}

fn travel_direction(direction: char, value: isize, coordinates: (isize, isize)) -> (isize, isize) {
    match direction {
        'N' => (coordinates.0, coordinates.1 + value),
        'S' => (coordinates.0, coordinates.1 - value),
        'E' => (coordinates.0 + value, coordinates.1),
        'W' => (coordinates.0 - value, coordinates.1),
        _ => panic!("Unexpected direction"),
    }
}

fn turn(face_direction: char, value: isize, action: char) -> char {
    let directions = ['N', 'E', 'S', 'W'];

    let turn_steps = if action == 'R' {
        value / 90
    } else {
        -1 * value / 90
    };
    let current_direction_index = directions
        .iter()
        .position(|d| d == &face_direction)
        .unwrap() as isize;
    let mod_index = (current_direction_index + turn_steps).rem_euclid(4);
    directions[mod_index as usize]
}

fn rotate_waypoint(waypoint: (isize, isize), value: isize, action: char) -> (isize, isize) {
    if value == 180 {
        return (-1 * waypoint.0, -1 * waypoint.1);
    } else if (value == 90 && action == 'L') || (value == 270 && action == 'R') {
        return (-1 * waypoint.1, waypoint.0);
    } else {
        return (waypoint.1, -1 * waypoint.0);
    }
}

#[cfg(test)]
mod tests {
    use crate::y2020::day12::*;

    #[test]
    fn test_navigate() {
        let ex = vec![
            "F10".to_string(),
            "N3".to_string(),
            "F7".to_string(),
            "R90".to_string(),
            "F11".to_string(),
        ];

        assert_eq!(navigate(&ex), 25);
    }

    #[test]
    fn test_navigate_waypoint() {
        let ex = vec![
            "F10".to_string(),
            "N3".to_string(),
            "F7".to_string(),
            "R90".to_string(),
            "F11".to_string(),
        ];

        assert_eq!(navigate_waypoint(&ex), 286);
    }
}
