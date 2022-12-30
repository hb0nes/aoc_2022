use std::num::IntErrorKind;

use thiserror::Error;

pub mod file_reading;

#[derive(Error, Debug)]
#[error("invalid puzzle input {line:?} on line {line_num:?}, because of {kind:?}")]
pub struct FileReaderIntError {
    line: String,
    line_num: i32,
    kind: IntErrorKind,
}

impl FileReaderIntError {
    pub fn kind(&self) -> &IntErrorKind {
        &self.kind
    }
}
