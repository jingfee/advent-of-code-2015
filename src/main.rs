use crate::solver::Solver;
use std::env;

mod days;
mod solver;

fn main() {
    let day = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("1"))
        .parse()
        .unwrap_or(1);
    solve_day(day);
}

fn solve_day(day: u8) {
    match day {
        1 => days::day01::Problem {}.solve(day),
        _ => println!("Day {} not yet implemented", day),
    }
}
