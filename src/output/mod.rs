use crate::Person;
use anyhow::Context;
use anyhow::Result;
use chrono::Local;
use comfy_table::Table;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tracing::info;

pub fn json_history(pairs: &[(Person, Person)], dir: PathBuf) -> Result<()> {
    // serialize to json and save
    let json = serde_json::to_string_pretty(&pairs)?;
    let date_time = Local::now();
    let formatted = format!("{}", date_time.format("%Y%m%d_%H%M%S"));
    let filename = format!("{formatted}.json");

    let mut path = PathBuf::new();
    path.push(dir);
    if !path.exists() {
        std::fs::create_dir_all(&path).context("Creating output directory.")?;
    }
    path.push(filename);

    let mut file = File::create(&path)?;
    file.write_all(json.as_bytes())?;
    info!("Saved history file to {path:?}");
    Ok(())
}

pub fn print_table(pairs: &[(Person, Person)]) {
    // now print the pairs
    let mut table = Table::new();
    for pair in pairs {
        table.add_row(vec![pair.0.clone(), pair.1.clone()]);
    }
    println!("{table}");
}
