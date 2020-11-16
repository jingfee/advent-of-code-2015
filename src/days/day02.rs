use crate::solver::Solver;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub struct Present {
    dimensions: Vec<u32>,
    sides: Vec<u32>,
}

impl Present {
    fn new(l: u32, w: u32, h: u32) -> Present {
        Present {
            dimensions: vec![l, h, w],
            sides: vec![l * w, w * h, h * l],
        }
    }

    fn square_feet_wrapping_paper(&self) -> u32 {
        let surface_area: u32 = self.sides.iter().map(|s| 2 * s).sum();
        let mut cloned_sides = self.sides.to_vec();
        cloned_sides.sort();
        surface_area + cloned_sides[0]
    }

    fn ribbon_length(&self) -> u32 {
        let mut cloned_dimensions = self.dimensions.to_vec();
        cloned_dimensions.sort();
        let length_wrap: u32 = cloned_dimensions.iter().take(2).map(|d| d*2).sum();
        let cube = self.dimensions[0] * self.dimensions[1] * self.dimensions[2];
        length_wrap + cube
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Present>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse_input(&self, file: File) -> Vec<Present> {
        let buf_reader = BufReader::new(file);
        buf_reader
            .lines()
            .map(|l| {
                let l = l.unwrap();
                let row_split: Vec<&str> = l.split('x').collect();
                Present::new(
                    row_split[0].parse().unwrap(),
                    row_split[1].parse().unwrap(),
                    row_split[2].parse().unwrap(),
                )
            })
            .collect()
    }

    fn solve_part_one(&self, input: &Vec<Present>) -> u32 {
        input.iter().map(|p| p.square_feet_wrapping_paper()).sum()
    }

    fn solve_part_two(&self, input: &Vec<Present>) -> u32 {
        input.iter().map(|p| p.ribbon_length()).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day02::*;

    #[test]
    fn test_square_feet_wrapping_paper() {
        let ex1 = Present::new(2, 3, 4);
        let ex2 = Present::new(1, 1, 10);
        //let ex3 = Present::new(29, 13, 26);

        assert_eq!(ex1.square_feet_wrapping_paper(), 58);
        assert_eq!(ex2.square_feet_wrapping_paper(), 43);
    }

    #[test]
    fn test_ribbon_length() {
        let ex1 = Present::new(2, 3, 4);
        let ex2 = Present::new(1, 1, 10);
        //let ex3 = Present::new(29, 13, 26);

        assert_eq!(ex1.ribbon_length(), 34);
        assert_eq!(ex2.ribbon_length(), 14);
    }
}
