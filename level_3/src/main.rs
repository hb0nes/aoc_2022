mod rucksack;

extern crate log;

use anyhow::{Error, format_err, Result};
use aoc_lib::file_reading::FileReaderString;
use rucksack::Rucksacks;
use env_logger::Env;
use log::{info};

/// Gets a single common item for each elf group
fn rucksacks_common_item(rucksacks: &Rucksacks) -> Result<char, Error> {
    let mut common_items = rucksacks[0].clone().contents;
    for rucksack in rucksacks {
        common_items = &common_items & &rucksack.contents;
    }
    if common_items.len() != 1 {
        return Err(format_err!("did not find exclusively a single item common for the given rucksacks {:?} (intersection: {:?})", rucksacks, common_items));
    }
    common_items.drain().last()
        .ok_or(format_err!("did not find a single item common for the given rucksacks {:?} (intersection: {:?})", rucksacks, common_items))
}

/// Converts a rucksack item to its priority.
/// The priority system follows ASCII decimal codes, with some arbitrary offset
fn item_priority(item: char) -> Result<i32> {
    let ascii_to_score_upper = 38;
    let ascii_to_score_lower = 96;
    match item.is_uppercase() {
        true => Ok(item as i32 - ascii_to_score_upper),
        false => Ok(item as i32 - ascii_to_score_lower),
    }
}

/// Splits a string into two rucksack halves
/// cannot use a RuckSack directly to split, as element order needs to be preserved
fn ruckset_halves(rucksack: &String) -> Result<Rucksacks, Error> {
    let mut rucksacks: Rucksacks = vec![];
    let (first_half, second_half) = rucksack.split_at(rucksack.len() / 2);
    rucksacks.push(first_half.into());
    rucksacks.push(second_half.into());
    Ok(rucksacks)
}

/// Solves the puzzle
fn solve() -> Result<(i32, i32), Error> {
    let f = FileReaderString::new(PUZZLE_NAME)?;
    let mut total_priority_1 = 0;
    let mut total_priority_2 = 0;
    let mut rucksacks: Rucksacks = vec![];
    for rucksack_string_result in &f {
        let rucksack_string = rucksack_string_result?;
        let rucksack_halves = ruckset_halves(&rucksack_string)?;
        total_priority_1 += item_priority(rucksacks_common_item(&rucksack_halves)?)?;
        rucksacks.push(rucksack_string.into());
        if rucksacks.len() == 3 {
            total_priority_2 += item_priority(rucksacks_common_item(&rucksacks)?)?;
            rucksacks.clear();
        }
    }
    Ok((total_priority_1, total_priority_2))
}

const PUZZLE_NAME: &str = "input";

fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().filter_or("RUST_LOG", "info"));
    let (puzzle_answer_1, puzzle_answer_2) = solve()?;
    info!("\npuzzle answer 2.1: {}\npuzzle answer 2.2: {}",puzzle_answer_1, puzzle_answer_2);
    Ok(())
}
