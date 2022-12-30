extern crate log;
extern crate core;

use anyhow::{Error, Result};
use aoc_lib::file_reading::FileReaderString;
use env_logger::Env;
use log::{info};

#[derive(Clone)]
struct SectionRanges {
    value: Vec<SectionRange>,
}

impl From<String> for SectionRanges {
    fn from(input_line: String) -> Self {
        let mut section_ranges: Vec<SectionRange> = vec![];
        for range in input_line.split(",") {
            let bounds = range.split("-")
                .take(2)
                .map(|s| s.parse().expect(&format!("incorrect puzzle input {}", input_line)))
                .collect::<Vec<u32>>();
            if !bounds.len() == 2 { panic!("incorrect puzzle input: {}", input_line) }
            section_ranges.push(SectionRange {
                lower_bound: bounds[0],
                upper_bound: bounds[1],
            });
        }
        SectionRanges {
            value: section_ranges
        }
    }
}

#[derive(Clone, Debug)]
struct SectionRange {
    lower_bound: u32,
    upper_bound: u32,
}

enum Overlap {
    Complete,
    Partial,
}
/**
Receives SectionRanges and checks against itself recursively to see
if any of the ranges encompass the other ranges, partially or completely.
During a complete match, we check if the lower & upper bounds are larger (or both smaller) than other elements.
A complete match is a subset of a partial match.
During a partial match, we check for each element if the lower or upper bound is somewhere between the other bounds.
*/
fn section_range_overlap(index: Option<usize>, mut section_ranges: SectionRanges, overlap: Overlap) -> bool {
    let index = index.unwrap_or(0);
    if index == section_ranges.value.len() { return false; }
    let section_range = &section_ranges.value.remove(index);
    for s in &section_ranges.value {
        if section_range.upper_bound >= s.upper_bound && section_range.lower_bound <= s.lower_bound { return true; }
        if section_range.upper_bound <= s.upper_bound && section_range.lower_bound >= s.lower_bound { return true; }
        if matches!(overlap, Overlap::Partial) {
            if (section_range.lower_bound >= s.lower_bound && section_range.lower_bound <= s.upper_bound)
                || (section_range.upper_bound >= s.lower_bound && section_range.upper_bound <= s.upper_bound)
            { return true; }
        }
    }
    section_range_overlap(Some(index), section_ranges, overlap)
}

/// Solves the puzzle
fn solve() -> Result<(u32, u32), Error> {
    let f = FileReaderString::new(PUZZLE_NAME)?;
    let mut overlap_counter_1 = 0u32;
    let mut overlap_counter_2 = 0u32;
    for line in &f {
        let line = line?;
        if section_range_overlap(None, line.clone().into(), Overlap::Complete) { overlap_counter_1 += 1; }
        if section_range_overlap(None, line.into(), Overlap::Partial) { overlap_counter_2 += 1; }
    }
    Ok((overlap_counter_1, overlap_counter_2))
}

const PUZZLE_NAME: &str = "input";

fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().filter_or("RUST_LOG", "info"));
    let (puzzle_answer_1, puzzle_answer_2) = solve()?;
    info!("\npuzzle answer 2.1: {}\npuzzle answer 2.2: {}",puzzle_answer_1, puzzle_answer_2);
    Ok(())
}
