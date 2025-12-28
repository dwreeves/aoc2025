use std::cmp::{max, min, Ordering};
use std::path::PathBuf;
use std::fs::File;
use std::io;
use std::io::{BufRead};
use std::str::FromStr;
use std::fmt;
use clap::Args;

#[derive(Args)]
#[command(
    about,
    long_about = "Cafeteria"
)]
pub(crate) struct Command {
    #[arg(name = "INPUT")]
    path: PathBuf
}

#[derive(Debug, Clone, Copy)]
struct IngredientIdRange {
    lower: i64,
    upper: i64
}

impl Ord for IngredientIdRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.lower.cmp(&other.lower)
            .then(self.upper.cmp(&other.upper))
    }
}

impl PartialOrd for IngredientIdRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for IngredientIdRange {
    fn eq(&self, other: &Self) -> bool {
        self.lower == other.lower && self.upper == other.upper
    }
}

impl Eq for IngredientIdRange {}

impl IngredientIdRange {
    fn merge(&self, other: &Self) -> Option<Self> {
        if self.lower > other.upper || other.lower > self.upper {
            None
        }
        else {
            Some(Self {
                lower: min(self.lower, other.lower),
                upper: max(self.upper, other.upper)
            })
        }
    }
}

impl FromStr for IngredientIdRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once("-") {
            Some((l, u)) => {
                let lower: i64 = l.parse().unwrap();
                let upper: i64 = u.parse().unwrap();
                Ok(Self { lower, upper })
            }
            None => {
                Err("Unable to parse IngredientIdRange".to_string())
            }
        }
    }
}

impl fmt::Display for IngredientIdRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use write! to format the output into the formatter f
        write!(f, "IngredientIdRange {{ lower: {}, upper: {} }}", self.lower, self.upper)
    }}


fn read_file(filename: &PathBuf) -> Result<(Vec<IngredientIdRange>, Vec<i64>), String> {
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let mut id_ranges: Vec<Option<IngredientIdRange>> = vec![];
    let mut ids: Vec<i64> = vec![];
    let mut iterator = reader.lines();
    // use reference to the iterator rather than directly
    // this allows us to resume iteration from where we left off
    for row in iterator.by_ref() {
        let txt = row.unwrap();
        if txt == "" {
            break
        }
        let id_range = IngredientIdRange::from_str(&txt)?;
        id_ranges.push(Some(id_range))
    }
    for row in iterator.by_ref() {
        let ingredient_id = row.unwrap().parse().unwrap();
        ids.push(ingredient_id);
    }
    id_ranges.sort();
    let mut i: usize = 0;
    let mut j: usize = 1;
    while max(i, j) < id_ranges.len() {
        match (id_ranges[i], id_ranges[j]) {
            (Some(first), Some(second)) => {
                match first.merge(&second) {
                    Some(mrg) => {
                        id_ranges[i] = Some(mrg);
                        id_ranges[j] = None;
                        j += 1;
                    }
                    None => {
                        i += 1;
                        j += 1;
                    }
                }
            }
            (Some(_first), None) => {
                panic!("This shouldn't happen lol");
            }
            (None, Some(_second)) => {
                i += 1;
                if i >= j {
                    j = i + 1;
                }
            }
            (None, None) => {
                i += 1;
                j += 1;
            }
        }
    }
    // println!("{:?}", id_ranges);
    Ok(
        (
            id_ranges
                .into_iter()
                .filter_map(|x| x)
                .collect(),
            ids
        )
    )
}

pub(crate) fn part1(command: &Command) {
    let (id_ranges, ids) = read_file(&command.path).unwrap();
    let mut count: i32 = 0;
    for id in ids {
        let pos = id_ranges.partition_point(|x| x.upper < id);
        if pos >= id_ranges.len() {
            continue
        }
        count += (id >= id_ranges[pos].lower && id <= id_ranges[pos].upper) as i32;
    }
    println!("{}", count);
}

pub(crate) fn part2(command: &Command) {
    let (id_ranges, _ids) = read_file(&command.path).unwrap();
    let count: i64 = id_ranges.iter().map(|x| x.upper - x.lower + 1).sum();
    println!("{}", count);
}
