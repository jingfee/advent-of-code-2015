use crate::solver::Solver;
use std::env;
extern crate pcre2;

mod solver;
mod y2015;
mod y2020;

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
            7 => y2015::day07::Problem {}.solve(&year, &day),
            8 => y2015::day08::Problem {}.solve(&year, &day),
            9 => y2015::day09::Problem {}.solve(&year, &day),
            10 => y2015::day10::Problem {}.solve(&year, &day),
            11 => y2015::day11::Problem {}.solve(&year, &day),
            12 => y2015::day12::Problem {}.solve(&year, &day),
            13 => y2015::day13::Problem {}.solve(&year, &day),
            _ => println!("Day {} not yet implemented", day),
        },
        2020 => match day {
            1 => y2020::day01::Problem {}.solve(&year, &day),
            2 => y2020::day02::Problem {}.solve(&year, &day),
            3 => y2020::day03::Problem {}.solve(&year, &day),
            4 => y2020::day04::Problem {}.solve(&year, &day),
            5 => y2020::day05::Problem {}.solve(&year, &day),
            6 => y2020::day06::Problem {}.solve(&year, &day),
            7 => y2020::day07::Problem {}.solve(&year, &day),
            8 => y2020::day08::Problem {}.solve(&year, &day),
            9 => y2020::day09::Problem {}.solve(&year, &day),
            10 => y2020::day10::Problem {}.solve(&year, &day),
            11 => y2020::day11::Problem {}.solve(&year, &day),
            12 => y2020::day12::Problem {}.solve(&year, &day),
            13 => y2020::day13::Problem {}.solve(&year, &day),
            14 => y2020::day14::Problem {}.solve(&year, &day),
            15 => y2020::day15::Problem {}.solve(&year, &day),
            16 => y2020::day16::Problem {}.solve(&year, &day),
            17 => y2020::day17::Problem {}.solve(&year, &day),
            18 => y2020::day18::Problem {}.solve(&year, &day),
            19 => y2020::day19::Problem {}.solve(&year, &day),
            20 => y2020::day20::Problem {}.solve(&year, &day),
            21 => y2020::day21::Problem {}.solve(&year, &day),
            22 => y2020::day22::Problem {}.solve(&year, &day),
            _ => println!("Day {} not yet implemented", day),
        },
        _ => println!("Year {} not yet implemented", year),
    }
}
