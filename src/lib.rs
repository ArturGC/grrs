use anyhow::{Context, Result};

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Reader {
    path: std::path::PathBuf,
    reader: BufReader<File>,
}
impl Reader {
    pub fn new(path: &std::path::PathBuf) -> Result<Reader, anyhow::Error> {
        File::open(&path)
            .map(|f| Reader {
                path: path.clone(),
                reader: BufReader::new(f),
            })
            .with_context(|| format!("could not read from file `{}`", path.display()))
    }

    pub fn get_line(&mut self) -> Result<String, anyhow::Error> {
        let mut line = String::new();

        self.reader
            .read_line(&mut line)
            .map(|_| line)
            .with_context(|| format!("could not read line from file `{}`", self.path.display()))
    }
}

pub struct Grep {
    pattern: String,
}
impl Grep {
    pub fn new(pattern: &str) -> Grep {
        Grep {
            pattern: String::from(pattern),
        }
    }

    pub fn check(&self, line: &str, mut writer: impl std::io::Write) {
        let line = line.trim_end();

        if line.contains(&self.pattern) {
            let _ = writeln!(writer, "{}", line);
        }
    }
}
