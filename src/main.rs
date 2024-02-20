use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser)]
struct CLI {
    pattern: String,
    path: std::path::PathBuf,
}

struct Reader {
    path: std::path::PathBuf,
    reader: BufReader<File>,
}
impl Reader {
    fn new(path: &std::path::PathBuf) -> Result<Reader, anyhow::Error> {
        File::open(&path)
            .map(|f| Reader {
                path: path.clone(),
                reader: BufReader::new(f),
            })
            .with_context(|| format!("could not read from file `{}`", path.display()))
    }
    fn get_line(&mut self) -> Result<String, anyhow::Error> {
        let mut line = String::new();

        self.reader
            .read_line(&mut line)
            .map(|_| line)
            .with_context(|| format!("could not read line from file `{}`", self.path.display()))
    }
}

struct Grep {
    pattern: String,
}
impl Grep {
    fn new(pattern: &String) -> Grep {
        Grep {
            pattern: pattern.clone(),
        }
    }
    fn check(&self, line: String) {
        let data = line.trim_end();

        if data.contains(&self.pattern) {
            println!("MATCH: {}", data);
        } else {
            println!("FAIL: {}", data);
        };
    }
}

fn main() -> Result<()> {
    let args = CLI::parse();

    let grep = Grep::new(&args.pattern);
    let mut reader = Reader::new(&args.path)?;

    loop {
        let line = reader.get_line()?;

        if line.len() != 0 {
            grep.check(line);
        } else {
            break;
        };
    }

    Ok(())
}
