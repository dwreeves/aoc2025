use clap::Args;
use std::{fs, io};
use std::io::{BufRead};
use std::path::PathBuf;

#[derive(Args)]
#[command(
    about,
    long_about = "Laboratories"
)]
pub(crate) struct Command {
    #[arg(name = "INPUT")]
    path: PathBuf
}

pub(crate) fn main(command: &Command) {

    let file = fs::File::open(&command.path).unwrap();
    let mut split_count: i32 = 0;

    let v = io::BufReader::new(&file)
        .lines()
        .map(|row| { row.unwrap().chars().collect::<Vec<char>>() })
        .fold(vec![], |mut state: Vec<i64>, next| {
            for i in 0..next.len() {
                match (state.get(i), next[i]) {
                    (Some(c), '^') if c.gt(&0) => {
                        split_count += 1;
                        state[i - 1] += state[i];
                        state[i + 1] += state[i];
                        state[i] = 0;
                    }
                    (_, 'S') => {
                        state = vec![0; next.len()];
                        state[i] = 1;
                    }
                    _ => ()
                }
            }
            state
        });

    println!("Part 1 = {}", split_count);
    println!("Part 2 = {}", v.iter().sum::<i64>());

}
