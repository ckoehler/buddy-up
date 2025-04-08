use anyhow::Result;
use buddy_up_lib::History;
use buddy_up_lib::People;
use clap::{Parser, Subcommand};
use std::fs::File;
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

    let f = File::open(input)?;
    let people = People::from_csv(f)?;

    // generate history from history directory (which contains the pairing files)
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
    buddy_up_lib::save_history(&pairs, &output_dir)?;

    // now print the pairs
    println!("{}", buddy_up_lib::print_table(pairs));
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
#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use tempfile::TempDir;

    #[test]
    fn test_cli_parsing() {
        let cli =
            Cli::try_parse_from(["app", "pair", "-i", "people.csv", "-o", "output_dir"]).unwrap();

        match cli.command {
            Commands::Pair { input, output_dir } => {
                assert_eq!(input, PathBuf::from("people.csv"));
                assert_eq!(output_dir, PathBuf::from("output_dir"));
            }
        }

        assert_eq!(cli.verbose, 0);
    }

    #[test]
    fn test_cli_verbose_flags() {
        let cli =
            Cli::try_parse_from(["app", "-v", "pair", "-i", "people.csv", "-o", "output_dir"])
                .unwrap();
        assert_eq!(cli.verbose, 1);

        let cli =
            Cli::try_parse_from(["app", "-vv", "pair", "-i", "people.csv", "-o", "output_dir"])
                .unwrap();
        assert_eq!(cli.verbose, 2);
    }

    #[test]
    fn test_pair_function() -> Result<()> {
        // Set up temporary directory
        let temp_dir = TempDir::new()?;
        let history_dir_path = temp_dir.path();

        // Create a temporary input file with test data
        let input_file = assert_fs::NamedTempFile::new("people.csv")?;
        input_file.write_str("1,Alice\n2,Bob\n3,Charlie\n4,David\n")?;

        // Run the pair function
        pair(input_file.path(), history_dir_path)?;

        // Verify a history file was created in the output directory
        let files = std::fs::read_dir(history_dir_path)?
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        assert!(!files.is_empty(), "Expected history files to be created");

        Ok(())
    }

    #[test]
    fn test_initialize_logging() {
        // Simply verify it doesn't panic
        initialize_logging(tracing::Level::INFO);
        // No assertions needed - just confirming it runs without error
    }

    #[test]
    fn test_pair_with_nonexistent_input() {
        let temp_dir = TempDir::new().unwrap();
        let result = pair(Path::new("/nonexistent/file.csv"), temp_dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_pair_with_invalid_csv() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let input_file = assert_fs::NamedTempFile::new("invalid.csv")?;
        input_file.write_str("invalid csv content")?;

        let result = pair(input_file.path(), temp_dir.path());
        assert!(result.is_err());

        Ok(())
    }
}
