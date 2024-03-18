use anyhow::{anyhow, Result};
use clap::Parser;
use serde_derive::Deserialize;
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Parser)]
struct Args {
    /// File to normalize
    #[arg(short, long = "input")]
    input_file: PathBuf,

    /// Output file
    #[arg(short, long = "output")]
    output_file: PathBuf,

    /// JSON configuration file
    #[arg(long = "config", default_value = "config.json")]
    config_path: PathBuf,
}

#[derive(Deserialize)]
struct Config {
    changes: HashMap<String, String>,
}

pub fn normalize(text: &str, changes: &HashMap<String, String>) -> String {
    changes
        .iter()
        .fold(text.to_string(), |acc, (original, replacement)| {
            acc.replace(original, replacement)
        })
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.input_file.exists() {
        return Err(anyhow!("Input file doesn't exist. Quitting..."));
    }

    let contents = fs::read_to_string(args.input_file)?;
    let config = fs::read_to_string(args.config_path)?;

    let tmp: Config = serde_json::from_str(&config)?;

    let result = normalize(contents.as_str(), &tmp.changes);
    fs::write(&args.output_file, &result)?;

    if result != contents {
        println!(
            "File normalization complete. Replaced {} occurrences.",
            result.len()
        );
    } else {
        println!("INFO: Modifications were applied, but nothing changed.");
    }

    Ok(())
}
