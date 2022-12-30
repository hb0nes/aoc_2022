mod rucksack;

extern crate log;

use anyhow::{Error, format_err, Result};
use aoc_lib::file_reading::FileReaderString;
use env_logger::Env;
use log::{info};


/// Solves the puzzle
fn solve() -> Result<(i32, i32), Error> {
    let f = FileReaderString::new(PUZZLE_NAME)?;
    for line in &f {
    }
    Ok((1,2))
}

const PUZZLE_NAME: &str = "input";

fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().filter_or("RUST_LOG", "info"));
    let (puzzle_answer_1, puzzle_answer_2) = solve()?;
    info!("\npuzzle answer 2.1: {}\npuzzle answer 2.2: {}",puzzle_answer_1, puzzle_answer_2);
    Ok(())
}
