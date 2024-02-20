use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn get_reader(path: &std::path::PathBuf) -> Result<BufReader<File>, anyhow::Error> {
    File::open(&path)
        .map(|f| BufReader::new(f))
        .with_context(|| format!("could no read from file `{}`", path.display()))
}

fn get_next_line(reader: &mut BufReader<File>) -> Result<String, anyhow::Error> {
    let mut line = String::new();

    reader
        .read_line(&mut line)
        .map(|_| line)
        .with_context(|| format!("could no read line from file"))
}

fn check_pattern(line: String, pattern: &String) {
    let line = line.trim_end();

    if line.contains(pattern) {
        println!("MATCH: {}", line);
    } else {
        println!("FAIL: {}", line);
    };
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut reader = get_reader(&args.path)?;

    loop {
        let line = get_next_line(&mut reader)?;

        if line.len() != 0 {
            check_pattern(line, &args.pattern);
        } else {
            break;
        };
    }

    Ok(())
}
