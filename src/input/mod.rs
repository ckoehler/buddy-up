use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tracing::info;

pub fn process(input: &Path) -> Result<HashMap<usize, String>> {
    let f = File::open(input)?;
    let reader = BufReader::new(f);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader);
    // TODO: check for unique ids somewhere here
    let mut people = HashMap::new();
    let mut tr_input_len = 0;
    for rec in rdr.records() {
        tr_input_len += 1;
        let r = rec?;
        let id = str::parse::<usize>(r.get(0).unwrap())?;
        let name = r.get(1).unwrap().to_string();

        people.insert(id, name);
    }

    // if these are not the same, the ids weren't unique
    if people.len() != tr_input_len {
        anyhow::bail!("Ids in the input are not unique. Aborting...");
    }
    if people.len() % 2 != 0 {
        anyhow::bail!("Number of inputs is not even. Can't currently handle that. Maybe add a dummy user with id 999999 for now?");
    }

    info!("Found {} records in input file.", people.len());

    Ok(people)
}
