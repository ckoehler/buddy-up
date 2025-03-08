use crate::BuddyError;
use crate::Pairs;
use crate::Person;
use chrono::Local;
use comfy_table::Table;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tracing::info;

/// Write the JSON history of this pairing to the given directory
pub fn save_history(pairs: &Pairs, dir: &str) -> Result<(), BuddyError> {
    // serialize to json and save
    let json = serde_json::to_string_pretty(&pairs)?;
    let date_time = Local::now();
    let formatted = format!("{}", date_time.format("%Y%m%d_%H%M%S"));
    let filename = format!("{formatted}.json");

    let mut path = PathBuf::new();
    path.push(dir);
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }
    path.push(filename);

    let mut file = File::create(&path)?;
    file.write_all(json.as_bytes())?;
    info!("Saved history file to {path:?}");
    Ok(())
}

/// Prints a pretty table of pairs.
///
/// If one pair includes the Evenizer with id usize::MAX, don't include the real user paired with
/// the Evenizer, and instead print a note below the table with that user not being paired.
pub fn print_table(pairs: Pairs) {
    // now print the pairs
    let mut table = Table::new();

    let mut unpaired: Option<Person> = None;

    // if a pair includes our Evenizer with id usize::MAX, don't pair that one and just print as not paired.
    for pair in pairs.inner() {
        if pair.1.id == usize::MAX {
            unpaired = Some(pair.0);
        } else if pair.0.id == usize::MAX {
            unpaired = Some(pair.1);
        } else {
            table.add_row(vec![pair.0.clone(), pair.1.clone()]);
        }
    }
    println!("{table}");
    if let Some(unpaired) = unpaired {
        println!("Not paired: {unpaired}");
    }
}
