use anyhow::{anyhow, Result};
use clap::Parser;
use serde_derive::Deserialize;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
};

#[derive(Parser)]
struct Args {
    #[arg(short, long = "input", help = "File to normalize")]
    input_file: PathBuf,

    #[arg(short, long = "output", help = "Output file")]
    output_file: PathBuf,

    #[arg(
        short,
        long = "config",
        help = "JSON configuration file",
        default_value = "config.json"
    )]
    config_path: PathBuf,
}

struct FileOperations;

impl FileOperations {
    pub fn read_file(path: String) -> Result<String> {
        let file = File::open(path)?;

        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        
        Ok(contents)
    }

    pub fn write(path: String, contents: &[u8]) -> Result<()> {
        let file = File::create(path)?;

        let mut reader = BufWriter::new(file);
        reader.write_all(contents)?;
        Ok(())
    }
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

fn load_config(config_path: &PathBuf) -> Result<Config> {
    let config_content = fs::read_to_string(config_path)?;
    let contents = serde_json::from_str(&config_content)?;
    Ok(contents)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let pathbuf_to_string = |path: PathBuf| -> String {
        path.to_string_lossy().into_owned()
    };

    if !args.input_file.exists() {
        return Err(anyhow!("Input file doesn't exist. Quitting..."));
    }

    let contents = FileOperations::read_file(pathbuf_to_string(args.input_file))?;

    let config = load_config(&args.config_path)?;
    let result = normalize(contents.as_str(), &config.changes);

    FileOperations::write(pathbuf_to_string(args.output_file), &result.as_bytes())?;

    if result != contents {
        println!(
            "File normalization complete. Replaced {} occurrences.",
            result.len()
        );
    } else {
        println!("Modifications were applied, but nothing changed.");
    }

    Ok(())
}
