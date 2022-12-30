extern crate log;

use anyhow::Result;
use aoc_lib::file_reading::FileReaderString;
use env_logger::Env;
use log::{debug, info};
use regex::Regex;

use crate::Outcome::{Draw, Loss, Win};
use crate::outcome::Outcome;
use crate::RockPaperScissorsMove::{Paper, Rock, Scissors};
use crate::rpsmove::RockPaperScissorsMove;

mod rpsmove;
mod outcome;

const PUZZLE_NAME: &str = "input";

fn determine_move(enemy_move: &RockPaperScissorsMove, desired_outcome: &Outcome) -> RockPaperScissorsMove {
    match enemy_move {
        Rock => match desired_outcome { &Win => Paper, &Loss => Scissors, _ => *enemy_move },
        Paper => match desired_outcome { &Win => Scissors, &Loss => Rock, _ => *enemy_move },
        Scissors => match desired_outcome { &Win => Rock, &Loss => Paper, _ => *enemy_move },
    }
}

fn determine_outcome(enemy_move: &RockPaperScissorsMove, friendly_move: &RockPaperScissorsMove) -> Outcome {
    match enemy_move {
        Rock => match friendly_move { &Paper => Win, &Scissors => Loss, _ => Draw },
        Paper => match friendly_move { &Scissors => Win, &Rock => Loss, _ => Draw },
        Scissors => match friendly_move { &Rock => Win, &Paper => Loss, _ => Draw },
    }
}

fn solve() -> Result<(i32, i32), anyhow::Error> {
    let mut ans_1 = 0;
    let mut ans_2 = 0;
    let line_validator = Regex::new(r"^([ABC]) ([XYZ])$")?;
    let f = FileReaderString::new(PUZZLE_NAME)?;
    for line in &f {
        let line = line?;
        let moves = line_validator.captures(&line)
            .ok_or(anyhow::format_err!("incorrect puzzle input \"{}\" at: {}:{}", line, f.filename(), f.line_num()))?;
        let (enemy_move, friendly_move, desired_outcome)
            : (RockPaperScissorsMove, RockPaperScissorsMove, Outcome)
            = (moves[1].into(), moves[2].into(), moves[2].into());
        let outcome = determine_outcome(&enemy_move, &friendly_move);
        let friendly_move_for_desired_outcome = determine_move(&enemy_move, &desired_outcome);
        let score_1 = outcome.value() + friendly_move.value();
        let score_2 = desired_outcome.value() + friendly_move_for_desired_outcome.value();
        debug!("aoc 2.1: {:<9} {:<9} {:<9} {}",enemy_move, friendly_move, outcome, score_1);
        debug!("aoc 2.2: {:<9} {:<9} {:<9} {}",enemy_move, desired_outcome, friendly_move_for_desired_outcome, score_2);
        ans_1 += score_1;
        ans_2 += score_2;
    }
    Ok((ans_1, ans_2))
}

fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().filter_or("RUST_LOG", "info"));
    let (puzzle_answer_1, puzzle_answer_2) = solve()?;
    info!(
        "\npuzzle answer 2.1: {}\npuzzle answer 2.2: {}",
        puzzle_answer_1, puzzle_answer_2
    );
    Ok(())
}
