use std::cell::Cell;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use anyhow::Context;

use crate::FileReaderIntError;

pub struct FileReaderInt {
    lines: Lines<BufReader<File>>,
    #[allow(dead_code)]
    line: String,
    line_num: i32,
}

impl FileReaderInt {
    pub fn new(filename: &str) -> Result<Self, anyhow::Error> {
        let f =
            File::open(filename).with_context(|| format!("could not open file '{}'", filename))?;
        let lines = BufReader::new(f).lines();
        Ok(Self {
            lines,
            line: String::new(),
            line_num: 0,
        })
    }
}

impl Iterator for FileReaderInt {
    type Item = Result<i32, FileReaderIntError>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next();
        let line = line?.ok()?;
        self.line_num += 1;
        return match line.parse::<i32>() {
            Ok(v) => Some(Ok(v)),
            Err(err) => Some(Err(FileReaderIntError {
                line,
                line_num: self.line_num,
                kind: err.kind().to_owned(),
            })),
        };
    }
}

pub struct FileReaderString {
    lines: RefCell<Lines<BufReader<File>>>,
    pub line_num: Cell<i32>,
    #[allow(dead_code)]
    pub filename: String,
}

impl FileReaderString {
    pub fn new(filename: &str) -> Result<Self, anyhow::Error> {
        let f =
            File::open(filename).with_context(|| format!("could not open file '{}'", filename))?;
        let lines = RefCell::new(BufReader::new(f).lines());
        let line_num = Cell::new(0);
        Ok(Self {
            lines: lines,
            line_num: line_num,
            filename: filename.to_string(),
        })
    }

    pub fn line_num(&self) -> i32 {
        self.line_num.get()
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }
}

impl Iterator for &FileReaderString {
    type Item = Result<String, anyhow::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self
            .lines
            .borrow_mut()
            .next()?
            .with_context(|| format!("could not read strings from {}", self.filename));
        self.line_num.set(self.line_num.get() + 1);
        match line {
            Ok(v) => Some(Ok(v)),
            Err(why) => Some(Err(why)),
        }
    }
}
