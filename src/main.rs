#![feature(rustc_private)]

use std::{
    fs::{read_to_string, write},
    path::PathBuf,
    collections::HashMap,
};
use serde_derive::{Serialize, Deserialize};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    /// Input file
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input_file: PathBuf,

    /// Output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output_file: PathBuf,

    /// JSON configuration file.
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    config_path: Option<PathBuf>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    changes: HashMap<String, String>,
}

pub fn normalize(text: &str, changes: &HashMap<String, String>) -> String {
    changes.iter().fold(text.to_string(), |acc, (original, replacement)| {
        acc.replace(original, replacement)
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_args();

    if !args.input_file.exists() {
        eprintln!("Input file doesn't exist. Quitting...");
        std::process::exit(1);
    }

    let contents = read_to_string(&args.input_file)?;

    let config_path = match args.config_path {
        Some(path) => path,
        None => PathBuf::from("config.json")
    };

    let config = read_to_string(config_path)?;
    let tmp: Config = serde_json::from_str(&config)?;

    let result = normalize(contents.as_str(), &tmp.changes);

    write(&args.output_file, result.clone())?;

    if result != contents {
        println!("File normalization complete. Replaced {} occurrences.", result.len());
    } else {
        println!("No modifications needed.");
    }

    Ok(())
}
