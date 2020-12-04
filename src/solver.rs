use std::fmt::Display;
use std::fs::File;
use std::io;

fn get_input_file_path(year: &u16, day: &u8) -> String {
    format!("input/{}/day{:02}.txt", year, day)
}

pub trait Solver {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn parse_input(&self, file: File) -> Self::Input;
    fn solve_part_one(&self, input: &mut Self::Input) -> Self::Output1;
    fn solve_part_two(&self, input: &mut Self::Input) -> Self::Output2;

    fn load_input(&self, file_path: String) -> io::Result<Self::Input> {
        let file = File::open(file_path)?;
        Ok(self.parse_input(file))
    }

    fn solve(&self, year: &u16, day: &u8) {
        let input_file_path = get_input_file_path(year, day);
        let mut input = self
            .load_input(input_file_path)
            .expect("Unable to open file");
        let part_1_solution = self.solve_part_one(&mut input);
        let part_2_solution = self.solve_part_two(&mut input);
        println!("Part 1: {}", part_1_solution);
        println!("Part 2: {}", part_2_solution);
    }
}
