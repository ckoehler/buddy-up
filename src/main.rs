mod algorithm;
mod input;
mod output;

use algorithm::History;
use algorithm::Person;
use algorithm::merge;
use anyhow::Context;
use anyhow::Result;
use clap::{Parser, Subcommand};
use glob::glob;
use std::path::Path;
use std::path::PathBuf;
use tracing::{debug, info};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Prints more or less info. Can be repeated. Use once for more output, twice for much more
    /// output.
    #[arg(global = true, short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    Pair {
        /// The path to a CSV file that defines the people input. Should be rows formatted like `id, name`.
        #[arg(short, long)]
        input: PathBuf,

        /// The directory where the output history is saved. Should probably be unique for each
        /// group of people. Will be created if it doesn't exist.
        #[arg(short, long)]
        output_dir: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // set up logging based on verbosity flag
    let level = if cli.verbose == 1 {
        tracing::Level::DEBUG
    } else if cli.verbose >= 2 {
        tracing::Level::TRACE
    } else {
        tracing::Level::INFO
    };
    initialize_logging(level);

    match &cli.command {
        Commands::Pair { input, output_dir } => {
            pair(input, output_dir)?;
        }
    }

    Ok(())
}

fn pair(input: &Path, output_dir: &Path) -> Result<()> {
    let output_dir = output_dir.to_string_lossy();

    let people = input::process(input)?;

    // generate history from pair files
    let mut history = History::new();
    // read pairs from dir of files
    let pattern = format!("{output_dir}/*.json");
    let mut tr_history_files = 0;
    for path in glob(&pattern).expect("Glob pattern works") {
        debug!("Reading history file {path:?}");
        let pairs = std::fs::read_to_string(path?)?;
        let pairs: Vec<(Person, Person)> = serde_json::from_str(&pairs)?;
        let pairs = pairs.iter().map(|p| (p.0.id, p.1.id)).collect();
        tr_history_files += 1;
        merge(&mut history, &pairs);
    }
    let tr_num_pairs = history.len();
    let tr_max_num_pairs = (people.len().pow(2) - people.len()) / 2;
    info!(
        "Read {tr_history_files} history files, found {tr_num_pairs} existing pairs (max possible: {tr_max_num_pairs})."
    );
    debug!("History min iterations: {}", history.min());
    debug!("History max iterations: {}", history.max());

    // the algorithm only operates on ids, so get those only. We can map them back to names for
    // output later.
    let people_ids = people.keys().copied().collect();
    let pairs = algorithm::pair(people_ids, &history);

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
    // TODO: that type gymnastic tho
    output::json_history(&pairs, output_dir.into_owned().into()).context("Saving history")?;

    // now print the pairs
    output::print_table(&pairs);
    Ok(())
}

fn initialize_logging(level: tracing::Level) {
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_file(false)
        .with_line_number(false)
        .with_target(false)
        .with_ansi(true)
        .init();
}
