use clap::{Parser, Subcommand};

pub mod aoc;

#[derive(Parser)]
struct Aoc {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(name = "day_1_trebuchet")]
    Day1(crate::aoc::day1::Task)
}

fn main() {
    let aoc = Aoc::parse();
   
    match aoc.command {
        Some(Commands::Day1(day1)) => day1.solve(),
        None => println!("No command provided"),
    }
}
