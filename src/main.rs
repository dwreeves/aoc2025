mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

use clap::{Parser};

#[derive(Parser)]
#[command(
    about,
    long_about = "I did AOC 2025 in Rust!"
)]
enum Cli {
    Day1Part1(day01::Command),
    Day1Part2(day01::Command),
    Day2Part1(day02::Command),
    Day2Part2(day02::Command),
    Day3Part1(day03::Command),
    Day3Part2(day03::Command),
    Day4Part1(day04::Command),
    Day4Part2(day04::Command),
    Day5Part1(day05::Command),
    Day5Part2(day05::Command),
    Day6Part1(day06::Command),
    Day6Part2(day06::Command),
}


fn main() {
    let cli = Cli::parse();

    match &cli {
        Cli::Day1Part1(cmd) => { day01::part1(cmd); }
        Cli::Day1Part2(cmd) => { day01::part2(cmd); }
        Cli::Day2Part1(cmd) => { day02::part1(cmd); }
        Cli::Day2Part2(cmd) => { day02::part2(cmd); }
        Cli::Day3Part1(cmd) => { day03::part1(cmd); }
        Cli::Day3Part2(cmd) => { day03::part2(cmd); }
        Cli::Day4Part1(cmd) => { day04::part1(cmd); }
        Cli::Day4Part2(cmd) => { day04::part2(cmd); }
        Cli::Day5Part1(cmd) => { day05::part1(cmd); }
        Cli::Day5Part2(cmd) => { day05::part2(cmd); }
        Cli::Day6Part1(cmd) => { day06::part1(cmd); }
        Cli::Day6Part2(cmd) => { day06::part2(cmd); }
    }
}