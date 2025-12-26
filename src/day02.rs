use std::path::PathBuf;
use std::fs::File;
use std::io;
use std::str::FromStr;
use clap::{Args};


#[derive(Args)]
#[command(
    about,
    long_about = "Gift Shop"
)]
pub(crate) struct Command {
    #[arg(name = "INPUT", help = "The input file to use")]
    path: PathBuf
}

#[derive(Debug)]
struct ProductIdRange {
    lower: i64,
    upper: i64
}

impl FromStr for ProductIdRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once("-") {
            Some((l, u)) => {
                let lower: i64 = l.parse().unwrap();
                let upper: i64 = u.parse().unwrap();
                Ok(ProductIdRange { lower, upper })
            }
            None => {
                Err("Unable to parse ProductIdRange".to_string())
            }
        }

    }
}

impl ProductIdRange {
    fn invalid_count(&self) -> i64 {
        let mut total = 0;

        for n in self.lower..=self.upper {
            let power = 10i64.pow(n.ilog10() / 2 + 1);
            let left = n / power;
            let right = n % power;
            if left == right {
                total += n;
            }
        }
        total
    }

    fn invalid_count_hard(&self) -> i64 {
        let mut total = 0;

        for n in self.lower..=self.upper {
            for p in 1..=n.ilog10() {
                if (n.ilog10() + 1) % p != 0 {
                    continue
                }
                let mut tmp = n;
                let power = 10i64.pow(p);
                let mut vec: Vec<i64> = vec![];
                while tmp > 0 {
                    vec.push(tmp % power);
                    tmp /= power;
                }
                if vec.len() >= 2 && vec.windows(2).all(|i| i[0] == i[1]) {
                    total += n;
                    break
                }
            }
        }
        total
    }
}

fn read_ids(filename: &PathBuf) -> Result<Vec<ProductIdRange>, String> {
    let file = File::open(filename).map_err(|e| e.to_string())?;
    let text = io::read_to_string(io::BufReader::new(file)).unwrap();
    let product_id_ranges = text.split(",")
        .map( |res| {
            ProductIdRange::from_str(res).unwrap()
        }).collect::<Vec<ProductIdRange>>();

    Ok(product_id_ranges)
}

pub(crate) fn part1(command: &Command) {

    let product_ids = read_ids(&command.path).unwrap();
    let total: i64 = product_ids.iter().map(|p|
        { p.invalid_count() }
    ).sum();

    println!("{}", total)
}

pub(crate) fn part2(command: &Command) {

    let product_ids = read_ids(&command.path).unwrap();
    let total: i64 = product_ids.iter().map(|p|
        { p.invalid_count_hard() }
    ).sum();

    println!("{}", total)
}