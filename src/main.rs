mod algorithm;
mod input;
mod output;

use glob::glob;
use spinners::{Spinner, Spinners};

use algorithm::merge;
use algorithm::History;
use algorithm::Person;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // TODO: get dir from command line
    let dir = "history";
    let input = "people.csv";

    // TODO: Traitify this
    let people = input::process(input)?;

    // generate history from pair files
    let mut history = History::new();
    // read pairs from dir of files
    let pattern = format!("{dir}/*.json");
    for path in glob(&pattern).expect("Glob pattern works") {
        let pairs = std::fs::read_to_string(path?)?;
        let pairs: Vec<(Person, Person)> = serde_json::from_str(&pairs)?;
        let pairs = pairs.iter().map(|p| (p.0.id, p.1.id)).collect();
        merge(&mut history, &pairs);
    }
    //dbg!(&history);
    //dbg!(&history.values());
    //let _ = &history.values().iter().for_each(|&v| assert!(v <= 5));

    let mut sp = Spinner::new(Spinners::Dots, String::new());
    // the algorithm only operates on ids, so get those only. We can map them back to names for
    // output later.
    let people_ids = people.keys().copied().collect();
    let pairs = algorithm::pair(people_ids, &history);
    sp.stop_with_symbol("\x1b[32m\x1b[0m");

    // put names back into the pairs for saving
    let pairs: Vec<(Person, Person)> = pairs
        .iter()
        .map(|(id1, id2)| {
            (
                Person::new(*id1, people.get(id1).unwrap().to_string()),
                Person::new(*id2, people.get(id2).unwrap().to_string()),
            )
        })
        .collect();

    // serialize to json and save
    let _ = output::json_history(&pairs, dir);

    // now print the pairs
    output::print_table(&pairs);
    Ok(())
}
