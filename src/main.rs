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
    Day7(crate::aoc::Task),
    Day8(crate::aoc::Task),
    Day9(crate::aoc::Task),
    Day10(crate::aoc::Task),
    Day11(crate::aoc::Task),
    Day12(crate::aoc::Task),
    Day13(crate::aoc::Task),
    Day14(crate::aoc::Task),
    Day15(crate::aoc::Task),
    Day16(crate::aoc::Task),
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
        Some(Commands::Day7(task)) => task.solve(&crate::aoc::day7::Day7, "6440".to_string(), "5905".to_string()),
        Some(Commands::Day8(task)) => task.solve(&crate::aoc::day8::Day8, "6".to_string(), "6".to_string()),
        Some(Commands::Day9(task)) => task.solve(&crate::aoc::day9::Day9, "114".to_string(), "2".to_string()),
        Some(Commands::Day10(task)) => task.solve(&crate::aoc::day10::Day10, "8".to_string(), "4".to_string()),
        Some(Commands::Day11(task)) => task.solve(&crate::aoc::day11::Day11, "374".to_string(), "82000210".to_string()),
        Some(Commands::Day12(task)) => task.solve(&crate::aoc::day12::Day12, "21".to_string(), "525152".to_string()),
        Some(Commands::Day13(task)) => task.solve(&crate::aoc::day13::Day13, "405".to_string(), "400".to_string()),
        Some(Commands::Day14(task)) => task.solve(&crate::aoc::day14::Day14, "136".to_string(), "64".to_string()),
        Some(Commands::Day15(task)) => task.solve(&crate::aoc::day15::Day15, "1320".to_string(), "145".to_string()),
        Some(Commands::Day16(task)) => task.solve(&crate::aoc::day16::Day16, "46".to_string(), "51".to_string()),
        None => println!("No command provided"),
    }
}
