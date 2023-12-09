use anyhow::{Error, Result};
use serde_derive::Deserialize;
use std::{
    fs,
    collections::HashMap,
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    /// File to normalize
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input_file: PathBuf,

    /// Output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output_file: PathBuf,

    /// JSON configuration file
    #[structopt(
        short = "c",
        long = "config",
        default_value = "config.json",
        parse(from_os_str)
    )]
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
    let args = Args::from_args();

    if !args.input_file.exists() {
        let error_msg = format!("Input file doesn't exist. Quitting...");
        return Err(Error::msg(error_msg));
    }

    let contents = fs::read_to_string(&args.input_file)?;
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
