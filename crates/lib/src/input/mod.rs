use crate::BuddyError;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::Read;
use tracing::info;

/// Abstraction over all the people you may want to pair up.
/// Give it a impl [`Read`], like a file, to get [`People`] back.
///
/// Example:
/// ```ignore
/// # use std::fs::File;
/// # use buddy_up_lib::People;
/// let f = File::open("people.csv")?;
/// let people = People::from_csv(f)?;
/// ```
#[derive(Debug)]
pub struct People(HashMap<usize, String>);

impl People {
    /// Reads people from a CSV file and creates a `People` struct from that.
    /// The expected format is rows of people like `id,name`.
    ///
    /// Example CSV:
    /// ```text
    /// 1,John
    /// 2,David
    /// ```
    pub fn from_csv<R: Read>(input: R) -> Result<Self, BuddyError> {
        let reader = BufReader::new(input);
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

        Ok(People(people))
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn not_even() {
        let csv = "1,Foo".as_bytes();
        let r = People::from_csv(csv);
        assert!(matches!(r, Err(BuddyError::NotEven)));
    }
    #[test]
    fn good() {
        let csv = "1,Foo\n2,Bar".as_bytes();
        let r = People::from_csv(csv);
        assert!(r.is_ok());
    }
    #[test]
    fn id_not_number() {
        let csv = "Baz,Foo\n2,Bar".as_bytes();
        let r = People::from_csv(csv);
        assert!(matches!(r, Err(BuddyError::IdNotANumber)));
    }
    #[test]
    fn id_not_unique() {
        let csv = "1,Foo\n1,Bar".as_bytes();
        let r = People::from_csv(csv);
        assert!(matches!(r, Err(BuddyError::IdsNotUnique)));
    }
    #[test]
    fn csv_format_wrong() {
        let csv = "1\n2".as_bytes();
        let r = People::from_csv(csv);
        assert!(matches!(r, Err(BuddyError::CsvFormatError)));
    }
}
