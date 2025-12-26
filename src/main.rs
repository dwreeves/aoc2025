mod day01;

use clap::{Parser};

#[derive(Parser)]
#[command(
    about,
    long_about = "I did AOC 2025 in Rust!"
)]
enum Cli {
    Day1Part1(day01::Command),
    Day1Part2(day01::Command)
}


fn main() {
    let cli = Cli::parse();

    match &cli {
        Cli::Day1Part1(cmd) => { day01::part1(cmd); }
        Cli::Day1Part2(cmd) => { day01::part2(cmd); }
    }
}