use clap::Args;
use std::path::PathBuf;
use std::fs;
use std::io::{self, BufRead};

#[derive(Args)]
#[command(
    about,
    long_about = "Trash Compactor"
)]
pub(crate) struct Command {
    #[arg(name = "INPUT")]
    path: PathBuf
}

fn transpose_matrix<T: Copy>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..matrix[0].len())
        .map(|i|
            (0..matrix.len())
                .map(|j| matrix[j][i])
                .collect()
        )
        .collect()
}

fn transpose_matrix_clone<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..matrix[0].len())
        .map(|i|
            (0..matrix.len())
                .map(|j| matrix[j][i].clone())
                .collect()
        )
        .collect()
}

pub(crate) fn part1(command: &Command) {

    let file = fs::File::open(&command.path).unwrap();
    let reader = io::BufReader::new(file);

    let chunks = reader.lines()
        .map(|s|
            s.unwrap()
                .split(" ")
                .filter(|&x| x != "")
                .map(str::to_string)
                .collect::<Vec<String>>()
        )
        .collect::<Vec<Vec<String>>>();

    let chunks = transpose_matrix_clone(chunks);

    let total: i128 = chunks.iter()
        .map(|x|
            match x.last().unwrap().as_str() {
                "*" => {
                    x[0..x.len() - 1].iter()
                        .map(|x| x.parse::<i128>().unwrap())
                        .reduce(|x, y| x * y)
                        .unwrap_or(0)
                },
                "+" => {
                    x[0..x.len() - 1].iter()
                        .map(|x| x.parse::<i128>().unwrap())
                        .sum()
                },
                _ => { panic!("this shouldn't happen"); }
            }
        )
        .sum();

    println!("{}", total);
}

pub(crate) fn part1_obnoxious(command: &Command) {
    // I made this deliberately obnoxious.
    // I think part1() above is a better implementation.
    // This was just for fun!
    println!("{}",
        transpose_matrix_clone(io::BufReader::new(
            fs::File::open(&command.path).unwrap()
        ).lines()
            .map(|s|
                s.unwrap()
                    .split(" ")
                    .filter(|&x| x != "")
                    .map(str::to_string)
                    .collect::<Vec<String>>()
            )
            .collect::<Vec<Vec<String>>>()).iter()
            .map(|x|
                match x.last().unwrap().as_str() {
                    "*" => {
                        x[0..x.len() - 1].iter()
                            .map(|x| x.parse::<i128>().unwrap())
                            .reduce(|x, y| x * y)
                            .unwrap_or(0)
                    },
                    "+" => {
                        x[0..x.len() - 1].iter()
                            .map(|x| x.parse::<i128>().unwrap())
                            .sum()
                    },
                    _ => { panic!("this shouldn't happen"); }
                }
            )
            .sum::<i128>()
    );
}

pub(crate) fn part2(command: &Command) {
    let mut total: i128 = 0;

    let file = fs::File::open(&command.path).unwrap();
    let reader = io::BufReader::new(file);

    let chars = reader.lines()
        .map(|s| s.unwrap().chars().collect())
        .collect::<Vec<Vec<char>>>();

    let max_length: usize = chars.iter()
        .map(Vec::len)
        .max()
        .unwrap_or(0);

    let chars = chars.iter()
        .map(|x|
            x.iter()
                .chain(vec![&' '; max_length - x.len()])
                .map(|&y| y)
                .collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut chars = transpose_matrix(chars);
    chars.reverse();

    let mut num_buf: Vec<i128> = vec![];

    for row in chars.iter() {
        let reduced_line = row
            .iter()
            .map(|s| s.to_string().replace(" ", ""))
            .collect::<String>();

        if reduced_line.is_empty() {
            continue
        }

        match reduced_line.chars().last() {
            Some('*') => {
                let s = &reduced_line[0..reduced_line.len() - 1];
                let n: i128 = s.parse().unwrap();
                num_buf.push(n);
                total += num_buf.iter().copied().reduce(|x, y| x * y).unwrap();
                num_buf = vec![];
            },
            Some('+') => {
                let s = &reduced_line[0..reduced_line.len() - 1];
                let n: i128 = s.parse().unwrap();
                num_buf.push(n);
                total += num_buf.iter().sum::<i128>();
                num_buf = vec![];
            },
            Some(_) => {
                let n: i128 = reduced_line.parse().unwrap();
                num_buf.push(n);
            },
            None => {}
        }
    }

    println!("{}", total);
}