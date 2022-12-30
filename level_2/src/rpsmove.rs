use std::fmt::{Display, Formatter};

use crate::rpsmove::RockPaperScissorsMove::{Paper, Rock, Scissors};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RockPaperScissorsMove {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissorsMove {
    pub fn value(&self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl From<&str> for RockPaperScissorsMove {
    fn from(ascii: &str) -> Self {
        match ascii {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => Rock,
        }
    }
}

impl Display for RockPaperScissorsMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rock => f.pad("Rock"),
            Paper => f.pad("Paper"),
            Scissors => f.pad("Scissors"),
        }
    }
}