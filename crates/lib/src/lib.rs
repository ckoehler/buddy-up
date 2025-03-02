mod algorithm;
mod input;
mod output;

use glob::GlobError;
use glob::PatternError;
use std::io;
use thiserror::Error;

pub use algorithm::history::*;
pub use algorithm::*;
pub use input::People;
pub use output::*;

use serde::Deserialize;
use serde::Serialize;

#[derive(Error, Debug)]
pub enum BuddyError {
    #[error("Couldn't read files.")]
    FileRead,

    #[error("Io Error")]
    IoError(#[from] io::Error),

    #[error("CSV Error")]
    CsvError(#[from] csv::Error),

    #[error("Error writing history as JSON.")]
    JsonError(#[from] serde_json::Error),

    #[error("CSV isn't formatted correctly. We expect rows of 'id,name', like '1,John'. ")]
    CsvFormatError,

    #[error("Given ID is not a number. Check input.")]
    IdNotANumber,

    #[error("The given IDs are not unique. Check input.")]
    IdsNotUnique,

    #[error("The number of people given is not even and cannot be paired.")]
    NotEven,

    #[error("Couldn't make a History path. Check the history directory.")]
    HistoryDirectoryError(#[from] GlobError),

    #[error("Error reading history, make sure there's nothing wrong with the directory name.")]
    PatternError(#[from] PatternError),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub id: usize,
    name: String,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Person {
    pub fn new(id: usize, name: String) -> Self {
        Self { id, name }
    }
}
