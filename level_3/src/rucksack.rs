use std::collections::HashSet;

pub type Rucksacks = Vec<Rucksack>;
pub type RucksackContents = HashSet<char>;

#[derive(Clone, Debug)]
pub struct Rucksack {
    pub contents: RucksackContents,
}

impl From<String> for Rucksack {
    fn from(value: String) -> Self {
        Rucksack { contents: value.chars().collect::<HashSet<char>>() }
    }
}

impl From<&str> for Rucksack {
    fn from(value: &str) -> Self {
        Rucksack { contents: value.chars().collect::<HashSet<char>>() }
    }
}

impl From<Vec<char>> for Rucksack {
    fn from(value: Vec<char>) -> Self {
        Rucksack { contents: value.iter().cloned().collect::<HashSet<char>>() }
    }
}