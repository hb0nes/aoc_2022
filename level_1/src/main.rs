extern crate log;

use std::num::IntErrorKind;

use anyhow::Result;
use aoc_lib::file_reading::FileReaderInt;
use env_logger::Env;
use log::{info};

const PUZZLE_NAME: &str = "input";

// solve returns the highest amount of calories for one elf
// and the sum of the top 3 of elves
fn solve() -> Result<(i32, i32), anyhow::Error> {
    let file_reader_int = FileReaderInt::new(PUZZLE_NAME)?;
    let mut elf_calories: Vec<i32> = vec![];
    let mut total_calorie_count = 0;
    for calorie_count in file_reader_int {
        match calorie_count {
            Ok(calorie_count) => total_calorie_count += calorie_count,
            Err(why) => {
                match why.kind() {
                    &IntErrorKind::Empty => {
                        elf_calories.push(total_calorie_count);
                        total_calorie_count = 0;
                    }
                    _ => return Err(why.into())
                }
            }
        }
    }
    elf_calories.sort_unstable();
    elf_calories.reverse();
    Ok((elf_calories[0], elf_calories[0..3].iter().sum()))
}

fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().filter_or("RUST_LOG", "info"));
    let (puzzle_answer_1, puzzle_answer_2) = solve()?;
    info!("\npuzzle answer 1.1: {}\npuzzle answer 1.2: {}", puzzle_answer_1, puzzle_answer_2);
    Ok(())
}