use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub fn process(input: &str) -> Result<HashMap<usize, String>> {
    let f = File::open(input)?;
    let reader = BufReader::new(f);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader);
    // TODO: check for unique ids somewhere here
    let mut people = HashMap::new();
    for rec in rdr.records() {
        let r = rec?;
        let id = str::parse::<usize>(r.get(0).unwrap())?;
        let name = r.get(1).unwrap().to_string();

        people.insert(id, name);
    }
    assert_eq!(people.len() % 2, 0);

    Ok(people)
}
