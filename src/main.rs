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
    Day2(crate::aoc::Task)
}

fn main() {
    let aoc = Aoc::parse();

    match aoc.command {
        Some(Commands::Day1(task)) => task.solve(&crate::aoc::day1::Day1, "142".to_string(), "281".to_string()),
        Some(Commands::Day2(task)) => task.solve(&crate::aoc::day2::Day2, "8".to_string(), "".to_string()),
        None => println!("No command provided"),
    }
}
