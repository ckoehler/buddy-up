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
#[derive(Clone, Debug, Default)]
pub struct People {
    people: HashMap<usize, String>,
    evenizer: bool,
}

impl People {
    /// Reads people from a CSV file and creates a `People` struct from that.
    /// The expected format is rows of people like `id,name`.
    ///
    /// Example CSV:
    /// ```text
    /// 1,John
    /// 2,David
    /// ```
    ///
    /// If the given input doesn't contain an even number of people, we will add our own with id
    /// `usize::MAX`, so that id is reserved.
    /// Having that extra user to make it even will keep the algorithm working, so that someone
    /// will be "paired up" with our evenizer, which really means that person won't get paired.
    /// The beautiful thing is that the algorithm will try and not repeat pairs, which now includes
    /// the evenizer, so the same person will not end up getting not paired all the time.
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
        let ret = if people.len() % 2 != 0 {
            people.insert(usize::MAX, "EVENIZER".to_string());
            tracing::warn!(
                "Input people are not even in number, so we can't pair everyone. One person will be left unpaired."
            );
            People {
                people,
                evenizer: true,
            }
        } else {
            People {
                people,
                evenizer: false,
            }
        };

        info!("Found {} records in input file.", ret.len());

        Ok(ret)
    }

    pub fn len(&self) -> usize {
        if self.has_evenizer() {
            self.people.len() - 1
        } else {
            self.people.len()
        }
    }

    /// Whether this People set has our evenizer user to make the count even.
    pub fn has_evenizer(&self) -> bool {
        self.evenizer
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub(crate) fn as_ids(&self) -> Vec<usize> {
        self.people.keys().copied().collect()
    }

    pub(crate) fn name_from_id(&self, id: usize) -> Option<String> {
        Some(self.people.get(&id)?.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // we'll add our evenizer, but keep the length the same as the original data
    #[test]
    fn not_even() {
        let csv = "1,Foo".as_bytes();
        let r = People::from_csv(csv);
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(r.len(), 1);
        assert!(r.has_evenizer());
    }
    #[test]
    fn good() {
        let csv = "1,Foo\n2,Bar".as_bytes();
        let r = People::from_csv(csv);
        assert!(r.is_ok());
        assert_eq!(r.unwrap().len(), 2);
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
