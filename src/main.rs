use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use globset::Glob;

use async_walkdir::WalkDir;
use futures_lite::stream::StreamExt;

use log::{LevelFilter, error, info};
use mlua::Lua;
use simple_logger::SimpleLogger;
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};
use tokio::fs as tokio_fs;

mod lua_api;
mod replace_strings;
use replace_strings::{Config, replace_strings};

#[derive(Parser)]
#[clap(
    name = "filesculptor",
    about = "Modify files by replacing specific strings or using regular expressions.",
    version
)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Process files using CLI configuration
    Run(RunArgs),
    /// Execute processing through Lua scripting
    Lua(LuaArgs),
}

#[derive(Parser)]
struct RunArgs {
    #[clap(help = "Input file or directory")]
    input_path: PathBuf,

    #[clap(short, help = "Output file (for single file processing)")]
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

    #[clap(long, help = "Process files recursively")]
    recursive: bool,

    #[clap(long, help = "Filter files using a glob pattern")]
    filter: Option<String>,

    #[clap(long, help = "Invert the filter match")]
    invert: bool,
}

#[derive(Parser)]
struct LuaArgs {
    /// Path to Lua script file
    #[clap(help = "Path to Lua script file")]
    script: PathBuf,

    #[clap(short, long, help = "Enable verbose mode")]
    verbose: bool,
}

async fn process_file(args: &RunArgs, config: &Config, path: &Path) -> Result<PathBuf> {
    let contents = tokio_fs::read_to_string(path).await?;
    let result = replace_strings(&contents, &config.changes)?;

    if args.dry_run {
        info!(
            "Dry run mode: Changes that would be applied to {:?}:\n{}",
            path, result
        );
        return Ok(path.to_path_buf());
    }

    if args.backup {
        let backup_file = path.with_extension("bak");
        tokio_fs::copy(path, &backup_file).await?;
        info!("Backup created at {:?}", backup_file);
    }

    if args.interactive {
        let mut input = String::new();
        println!("Proposed changes for {:?}:\n{}", path, result);
        print!("Apply changes? (y/N): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            info!("Changes for {:?} were not applied.", path);
            return Ok(path.to_path_buf());
        }
    }

    if let Some(output_path) = &args.output_file {
        tokio_fs::write(output_path, result.as_bytes()).await?;
    } else {
        // Handle single file output to stdout
        print!("{}", result);
    }

    info!("Processed file: {:?}", path);
    Ok(path.to_path_buf())
}

async fn process_recursive(args: &RunArgs, config: &Config) -> Result<Vec<PathBuf>> {
    let filter_glob = args.filter.as_ref().map(|f| {
        Glob::new(f)
            .expect("Invalid glob pattern")
            .compile_matcher()
    });

    let mut processed_paths = Vec::new();

    let mut walkdir = WalkDir::new(&args.input_path);
    while let Some(entry) = walkdir.next().await {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            continue;
        }

        let matches = filter_glob.as_ref().map_or(true, |g| g.is_match(&path));
        let include = matches ^ args.invert;

        if include {
            let processed = process_file(args, config, &path).await?;
            processed_paths.push(processed);
        }
    }

    Ok(processed_paths)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Run(run_args) => run_cli(run_args).await,
        Command::Lua(lua_args) => run_lua(lua_args).await,
    }
}

async fn run_cli(args: RunArgs) -> Result<()> {
    SimpleLogger::new()
        .with_level(if args.verbose {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        })
        .env()
        .with_utc_timestamps()
        .init()?;

    if !args.input_path.exists() {
        error!("Input path doesn't exist: {:?}", args.input_path);
        bail!("Input path doesn't exist.");
    }

    let config = Config::load(&args.config_path)?;

    if args.recursive {
        process_recursive(&args, &config).await?;
    } else {
        if args.input_path.is_dir() {
            error!("Input path is a directory. Use --recursive to process directories.");
            bail!("Input path is a directory");
        }
        process_file(&args, &config, &args.input_path).await?;
    }
    Ok(())
}

async fn run_lua(args: LuaArgs) -> Result<()> {
    SimpleLogger::new()
        .with_level(if args.verbose {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        })
        .init()?;

    let lua = Lua::new();
    lua_api::init(&lua)?;

    let script = fs::read_to_string(&args.script)
        .with_context(|| format!("Failed to read Lua script: {:?}", args.script))?;

    lua.load(&script).exec()?;
    Ok(())
}
