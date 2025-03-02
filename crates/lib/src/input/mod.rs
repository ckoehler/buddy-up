use crate::BuddyError;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tracing::info;

pub struct People(HashMap<usize, String>);

impl People {
    pub fn from_csv(input: &Path) -> Result<Self, BuddyError> {
        Ok(Self(process(input)?))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub(crate) fn as_ids(&self) -> Vec<usize> {
        self.0.keys().copied().collect()
    }

    pub(crate) fn name_from_id(&self, id: usize) -> Option<String> {
        Some(self.0.get(&id)?.to_string())
    }
}

fn process(input: &Path) -> Result<HashMap<usize, String>, BuddyError> {
    let f = File::open(input)?;
    let reader = BufReader::new(f);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader);
    let mut people = HashMap::new();
    let mut tr_input_len = 0;
    for rec in rdr.records() {
        tr_input_len += 1;
        let r = rec?;
        let id = str::parse::<usize>(r.get(0).ok_or(BuddyError::CsvFormatError)?)
            .map_err(|_| BuddyError::IdNotANumber)?;
        let name = r.get(1).ok_or(BuddyError::CsvFormatError)?.to_string();

        people.insert(id, name);
    }

    // if these are not the same, the ids weren't unique
    if people.len() != tr_input_len {
        return Err(BuddyError::IdsNotUnique);
    }
    if people.len() % 2 != 0 {
        return Err(BuddyError::NotEven);
    }

    info!("Found {} records in input file.", people.len());

    Ok(people)
}
