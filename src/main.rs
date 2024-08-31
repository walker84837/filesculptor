use anyhow::{bail, Context, Result};
use clap::Parser;
use log::{error, info, LevelFilter};
use simple_logger::SimpleLogger;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

mod replace_strings;
use replace_strings::{replace_strings, Config};

#[derive(Parser)]
#[clap(
    name = "filesculptor",
    about = "Modify files by replacing specific strings or using regular expressions."
)]
struct Args {
    #[clap(help = "replace_strings")]
    input_file: PathBuf,

    #[clap(short, help = "Output file")]
    output_file: Option<PathBuf>,

    #[clap(short, help = "Configuration JSON file", default_value = "config.json")]
    config_path: PathBuf,

    #[clap(short, long, help = "Enable verbose mode")]
    verbose: bool,

    #[clap(
        long,
        help = "Perform a dry run without writing changes to the output file"
    )]
    dry_run: bool,

    #[clap(
        long,
        help = "Create a backup of the original file before making changes"
    )]
    backup: bool,

    #[clap(
        long,
        help = "Interactive mode to confirm changes before applying them"
    )]
    interactive: bool,
}

fn load_config(config_path: &PathBuf) -> Result<Config> {
    let config_content = fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read configuration file: {:?}", config_path))?;

    let config_ext = config_path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or_default();

    let config: Config = match config_ext {
        "json" => serde_json::from_str(&config_content)
            .with_context(|| "Failed to parse JSON configuration")?,
        _ => bail!("Unsupported configuration file format"),
    };

    Ok(config)
}

fn process_file(args: &Args, config: &Config) -> Result<()> {
    let contents = fs::read_to_string(&args.input_file)
        .with_context(|| format!("Failed to read input file: {:?}", &args.input_file))?;

    let result = replace_strings(contents.as_str(), &config.changes)?;

    if args.dry_run {
        info!("Dry run mode: Changes that would be applied:\n{}", result);
        return Ok(());
    }

    if args.backup {
        let backup_file = args.input_file.with_extension("bak");
        fs::copy(&args.input_file, &backup_file)
            .with_context(|| format!("Failed to create backup file: {:?}", backup_file))?;
        info!("Backup created at {:?}", backup_file);
    }

    if args.interactive {
        let mut input = String::new();

        println!("Proposed changes:\n{}", result);
        print!("Apply changes? (y/N): ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() != "y" {
            info!("Changes were not applied.");
            return Ok(());
        }
    }

    if let Some(path) = &args.output_file {
        fs::write(path, &result)
            .with_context(|| format!("Failed to write to output file: {:?}", &args.output_file))?;
    } else {
        print!("{result}");
    }

    if result != contents {
        info!("File normalization complete. Changes were applied.");
    } else {
        info!("Modifications were applied, but nothing changed.");
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    SimpleLogger::new()
        .with_level(if args.verbose {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        })
        .env()
        .with_utc_timestamps()
        .init()
        .unwrap();

    if !args.input_file.exists() {
        error!("Input file doesn't exist: {:?}", args.input_file);
        bail!("Input file doesn't exist.");
    }

    let config = load_config(&args.config_path)?;

    match process_file(&args, &config) {
        Ok(_) => info!("Processing completed successfully."),
        Err(e) => error!("Error during processing: {:?}", e),
    }

    Ok(())
}
