use clap::Parser;

#[derive(Parser)]
pub struct CLI {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    use grrs::*;

    let args = CLI::parse();
    let grep = Grep::new(&args.pattern);
    let mut reader = Reader::new(&args.path)?;

    loop {
        let line = reader.get_line()?;

        if line.len() != 0 {
            grep.check(&line, &mut std::io::stdout());
        } else {
            break;
        };
    }

    Ok(())
}
