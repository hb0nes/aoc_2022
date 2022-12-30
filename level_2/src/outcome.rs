use std::fmt::{Display, Formatter};

use crate::Outcome::{Draw, Loss, Win};

#[derive(PartialEq, Debug)]
pub enum Outcome {
    Loss,
    Draw,
    Win,
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => Loss,
        }
    }
}

impl Outcome {
    pub fn value(&self) -> i32 {
        match self {
            Loss => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

impl Display for Outcome {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Loss => f.pad("Loss"),
            Draw => f.pad("Draw"),
            Win => f.pad("Win"),
        }
    }
}