use std::path::PathBuf;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use clap::{Args};

#[derive(Args)]
#[command(
    about,
    long_about = "Secret Entrance"
)]
pub(crate) struct Command {
    #[arg(name = "INPUT", help = "The input file to use")]
    path: PathBuf
}

#[derive(Debug)]
struct Rotation {
    direction: char,
    amount: i32,
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let direction: char = s.chars().nth(0).unwrap();
        let amount: i32 = s.chars().skip(1).collect::<String>().parse().unwrap();
        Ok(Rotation { direction, amount })
    }
}

fn read_lines(filename: &PathBuf) -> Result<Vec<Rotation>, String> {
    let file = File::open(filename).map_err(|e| e.to_string())?;
    let reader = io::BufReader::new(file);
    let rotations: Vec<Rotation> = reader.lines().enumerate()
        .map( |(row_num, res)| {
            let line = match res {
                Ok(l) => l,
                Err(_e) => {
                    panic!("Unable to read row {}", row_num);
                }
            };

            // Parse the line into a Person struct
            match Rotation::from_str(&line) {
                Ok(rotation) => rotation,
                Err(_e) => {
                    panic!("Unable to parse line {}", row_num);
                }
            }
        }).collect::<Vec<Rotation>>();

    Ok(rotations)
}

pub(crate) fn part1(command: &Command) {
    let rotations = read_lines(&command.path).unwrap();

    let mut pos: i32 = 50;
    let mut count: i32 = 0;

    rotations.iter().for_each(|rotation| {
        match rotation.direction {
            'L' => { pos -= rotation.amount },
            'R' => { pos += rotation.amount },
            _ => panic!("Unknown direction: {}", rotation.direction),
        }
        // pos = (100 + (pos % 100)) % 100;
        pos = pos.rem_euclid(100);  // Thanks Clippy!
        if pos == 0 {
            count += 1;
        }
    });

    println!("{}", count);
}

pub(crate) fn part2(command: &Command) {
    let rotations = read_lines(&command.path).unwrap();

    let mut pos: i32 = 50;
    let mut count: i32 = 0;

    rotations.iter().for_each(|rotation| {
        count += rotation.amount / 100;
        match rotation.direction {
            'L' => {
                if pos != 0 && pos <= (rotation.amount % 100) {
                    count += 1;
                }
                pos -= rotation.amount
            },
            'R' => {
                if pos + (rotation.amount % 100) >= 100 {
                    count += 1;
                }
                pos += rotation.amount
            },
            _ => panic!("Unknown direction: {}", rotation.direction),
        }
        // pos = (100 + (pos % 100)) % 100;
        pos = pos.rem_euclid(100);  // Thanks Clippy!
    });

    println!("{}", count);
}
