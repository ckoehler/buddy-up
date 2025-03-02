mod algorithm;
mod input;
mod output;

pub use algorithm::history::*;
pub use algorithm::*;
pub use input::process;
pub use output::*;

use serde::Deserialize;
use serde::Serialize;

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
