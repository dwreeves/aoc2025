use std::path::PathBuf;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::FromStr;
use clap::{Args};


#[derive(Args)]
#[command(
    about,
    long_about = "Lobby"
)]
pub(crate) struct Command {
    #[arg(name = "INPUT", help = "The input file to use")]
    path: PathBuf
}

#[derive(Debug)]
struct BatteryRow {
    vec: Vec<i64>
}

impl FromStr for BatteryRow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Vec<i64> = s.chars().map(|c|
            c.to_string().parse().unwrap()
        ).collect();
        Ok(BatteryRow { vec })
    }
}

fn read_rows(filename: &PathBuf) -> Result<Vec<BatteryRow>, String> {
    let file = File::open(filename).map_err(|e| e.to_string())?;
    let reader = io::BufReader::new(file);

    let battery_rows = reader.lines()
        .map( |res| {
            match res {
                Ok(l) => BatteryRow::from_str(&l).unwrap(),
                Err(_e) => { panic!("Unable to read row"); }
            }
        }).collect::<Vec<BatteryRow>>();

    Ok(battery_rows)
}

pub(crate) fn part1(command: &Command) {
    let batteries = read_rows(&command.path).unwrap();
    let mut total = 0;

    for row in batteries.iter() {
        let mut highest_ten = -1;
        let mut highest_idx: usize = 0;

        for idx in 0..row.vec.len() - 1 {
            let b = row.vec[idx];
            if b > highest_ten {
                highest_ten = b;
                highest_idx = idx;
            }
        }

        let highest_one = row.vec[highest_idx + 1..row.vec.len()].iter().max().unwrap();

        total += highest_ten * 10 + highest_one;
    }
    println!("{}", total);
}

const ALLOWED_BATTERIES: usize = 12;

pub(crate) fn part2(command: &Command) {
    let batteries = read_rows(&command.path).unwrap();
    let mut total = 0;

    for row in batteries.iter() {
        let mut prev_idx: usize = 0;
        let mut digit_vec: Vec<i64> = vec![-1; ALLOWED_BATTERIES];
        let mut idx_vec: Vec<usize> = vec![0; ALLOWED_BATTERIES];

        for pos in 0..ALLOWED_BATTERIES {
            for idx in prev_idx..row.vec.len() - ALLOWED_BATTERIES + pos + 1 {
                let b = row.vec[idx];
                if b > digit_vec[pos] {
                    digit_vec[pos] = b;
                    idx_vec[pos] = idx;
                }
            }
            prev_idx = idx_vec[pos] + 1;
        }

        let ret: i64 = digit_vec.iter().enumerate()
            .map(|(idx, val)| {
                val * (10i64).pow((ALLOWED_BATTERIES - idx - 1) as u32)
            }).sum();
        total += ret;
    }
    println!("{}", total);
}
