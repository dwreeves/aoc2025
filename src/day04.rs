use std::path::PathBuf;
use std::fs::File;
use std::io::{self, BufRead};
use clap::Args;


#[derive(Args)]
#[command(
    about,
    long_about = "Printing Department"
)]
pub(crate) struct Command {
    #[arg(name = "INPUT", help = "The input file to use")]
    path: PathBuf
}

fn read_file(filename: &PathBuf) -> Result<Vec<Vec<bool>>, String> {
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let matrix : Vec<Vec<bool>> = reader.lines()
        .map( | row_result| {
            let line = row_result.unwrap();
            let mut row: Vec<bool> = vec![];
            for ch in line.chars() {
                row.push(match ch {
                    '@' => true,
                    _ => false
                })
            }
            row
        }).collect::<Vec<Vec<bool>>>();

    Ok(matrix)
}

fn adjacency(matrix: &Vec<Vec<bool>>, i: usize, j: usize) -> bool {
    let mut count = 0;
    for i_delta in i.saturating_sub(1)..=i+1 {
        for j_delta in j.saturating_sub(1)..=j+1 {
            if i_delta == i && j_delta == j {
                continue
            }
            match matrix.get(i_delta) {
                Some(row) => match row.get(j_delta) {
                    Some(cell) => { count += cell.clone() as i32; }
                    None => ()
                }
                None => ()
            }
        }
    }
    count < 4
}

pub(crate) fn part1(command: &Command) {
    let matrix = read_file(&command.path).unwrap();

    let mut count: i32 = 0;

    for i in 0..matrix.clone().len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] {
                count += adjacency(&matrix, i, j) as i32;
            }
        }
    }

    println!("{}", count);
}

pub(crate) fn part2(command: &Command) {
    let mut matrix = read_file(&command.path).unwrap();

    let mut count: i32 = 0;
    let mut prev_count = -1;

    while count != prev_count {
        prev_count = count;
        for i in 0..matrix.clone().len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] {
                    if adjacency(&matrix, i, j) {
                        count += 1;
                        matrix[i][j] = false;
                    }
                }
            }
        }
    }

    println!("{}", count);
}