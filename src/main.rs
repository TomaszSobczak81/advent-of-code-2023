use clap::{Parser, Subcommand};

pub mod aoc;

#[derive(Parser)]
struct Aoc {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Day1(crate::aoc::Task),
    Day2(crate::aoc::Task),
    Day3(crate::aoc::Task),
    Day4(crate::aoc::Task),
    Day5(crate::aoc::Task),
    Day6(crate::aoc::Task),
}

fn main() {
    let aoc = Aoc::parse();

    match aoc.command {
        Some(Commands::Day1(task)) => task.solve(&crate::aoc::day1::Day1, "142".to_string(), "281".to_string()),
        Some(Commands::Day2(task)) => task.solve(&crate::aoc::day2::Day2, "8".to_string(), "2286".to_string()),
        Some(Commands::Day3(task)) => task.solve(&crate::aoc::day3::Day3, "4361".to_string(), "467835".to_string()),
        Some(Commands::Day4(task)) => task.solve(&crate::aoc::day4::Day4, "13".to_string(), "30".to_string()),
        Some(Commands::Day5(task)) => task.solve(&crate::aoc::day5::Day5, "35".to_string(), "46".to_string()),
        Some(Commands::Day6(task)) => task.solve(&crate::aoc::day6::Day6, "288".to_string(), "71503".to_string()),
        None => println!("No command provided"),
    }
}
