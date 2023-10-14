use std::{error::Error, fs::write};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    /// Input file
    #[structopt(parse(from_os_str))]
    input_file: std::path::PathBuf,

    /// Output file
    #[structopt(parse(from_os_str))]
    output_file: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::from_args();

    if !args.input_file.exists() {
        eprintln!("Input file doesn't exist. Quitting...");
        return Ok(());
    }

    let mut contents = std::fs::read_to_string(&args.input_file)?;

    let normalized_text = replace_curly_quotes(&mut contents);

    write(&args.output_file, normalized_text)?;

    println!(
        "File normalization completed. Normalized text saved to {}.",
        args.output_file.display()
    );

    Ok(())
}

fn replace_curly_quotes(text: &str) -> String {
    text.replace('\u{201C}', "\"")
        .replace('\u{201D}', "\"")
        .replace('\u{2018}', "'")
        .replace('\u{2019}', "'")
        .replace('\u{2026}', "...")
        .replace('\u{2014}', "-")
        .replace('\u{2013}', "-")
        .replace('\u{ff02}', "\"")
        .replace('\u{00a0}', " ")
        .replace('\u{25cf}', "-")
}
