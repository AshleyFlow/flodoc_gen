use clap::Parser as ClapParser;
use parser::parse_content;
use std::{fs, path::PathBuf};
use util::{Error, ExpectedPath};

mod parser;
mod util;

#[derive(ClapParser)]
struct Cli {
    directory: PathBuf,
}

fn parse_file(path: PathBuf) -> Result<(), Error> {
    if let Err(error) = ExpectedPath::File.check_path(&path) {
        return Err(error);
    }

    println!("Parsing file '{}'", path.display());

    let content = {
        let utf8 = fs::read(path)?;
        String::from_utf8(utf8)?
    };

    parse_content(content);

    Ok(())
}

fn parse_path(path: PathBuf) -> Result<(), Error> {
    if let Err(error) = ExpectedPath::Dir.check_path(&path) {
        return Err(error);
    }

    println!("Parsing directory '{}'", path.display());

    for dir in path.read_dir()? {
        let Ok(entry) = dir else {
            continue;
        };

        let entry_path = entry.path();

        if entry_path.is_dir() {
            parse_path(entry_path)?;
        } else {
            parse_file(entry_path)?;
        }
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    if let Err(error) = parse_path(cli.directory) {
        println!(
            "Failed to parse directory\n Reason:\n \t{}",
            error.to_string().replace("\n", "\n \t")
        );
    }
}
