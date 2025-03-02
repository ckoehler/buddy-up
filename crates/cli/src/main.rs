use anyhow::Result;
use buddy_up_lib::History;
use buddy_up_lib::People;
use buddy_up_lib::{print_table, save_history};
use clap::{Parser, Subcommand};
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

fn pair(input: &Path, history_dir: &Path) -> Result<()> {
    let output_dir = history_dir.to_string_lossy();

    let people = People::from_csv(input)?;

    // generate history from pair files
    let history = History::from_dir(&output_dir)?;

    let tr_num_pairs = history.stats().pairs;
    let tr_max_num_pairs = (people.len().pow(2) - people.len()) / 2;
    let tr_history_files = history.stats().files_read;
    info!(
        "Read {tr_history_files} history files, found {tr_num_pairs} existing pairs (max possible: {tr_max_num_pairs})."
    );
    debug!("History min iterations: {}", history.min());
    debug!("History max iterations: {}", history.max());

    let pairs = buddy_up_lib::pair(people, &history);

    // serialize to json and save
    save_history(&pairs, &output_dir)?;

    // now print the pairs
    print_table(pairs);
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
