use crate::solver::Solver;
use std::env;

mod solver;
mod y2015;

fn main() {
    let (year, day) = parse_config();
    solve_day(year, day);
}

fn parse_config() -> (u16, u8) {
    let year = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("2020"))
        .parse()
        .unwrap_or(2020);
    let day = env::args()
        .nth(2)
        .unwrap_or_else(|| String::from("1"))
        .parse()
        .unwrap_or(1);

    (year, day)
}

fn solve_day(year: u16, day: u8) {
    match year {
        2015 => match day {
            1 => y2015::day01::Problem {}.solve(&year, &day),
            2 => y2015::day02::Problem {}.solve(&year, &day),
            3 => y2015::day03::Problem {}.solve(&year, &day),
            4 => y2015::day04::Problem {}.solve(&year, &day),
            5 => y2015::day05::Problem {}.solve(&year, &day),
            6 => y2015::day06::Problem {}.solve(&year, &day),
            _ => println!("Day {} not yet implemented", day),
        },
        _ => println!("Year {} not yet implemented", year),
    }
}
