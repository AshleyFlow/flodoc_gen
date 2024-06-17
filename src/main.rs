use clap::Parser as ClapParser;
use parser::parse_content;
use std::{fs, path::PathBuf, time::Instant};
use util::{Error, ExpectedPath};

mod parser;
mod util;

#[derive(ClapParser)]
struct Cli {
    directory: PathBuf,
    #[arg(short, long)]
    output_dir: Option<PathBuf>,
}

fn parse_file(path: PathBuf) -> Result<serde_json::Value, Error> {
    if let Err(error) = ExpectedPath::File.check_path(&path) {
        return Err(error);
    }

    println!("Parsing file '{}'", path.display());

    let content = {
        let utf8 = fs::read(&path)?;
        String::from_utf8(utf8)?
    };

    parse_content(content)
}

fn parse_path(custom_dir: Option<PathBuf>, path: PathBuf) -> Result<(), Error> {
    if let Err(error) = ExpectedPath::Dir.check_path(&path) {
        return Err(error);
    }

    println!("Parsing directory '{}'", path.display());

    for dir in path.read_dir()? {
        let Ok(entry) = dir else {
            continue;
        };

        let path = entry.path();

        if path.is_dir() {
            parse_path(custom_dir.clone(), path)?;
        } else {
            let json = parse_file(path.clone())?;
            let parsed = serde_json::to_string_pretty(&json)?;
            let parsed_path = { custom_dir.clone().unwrap_or(PathBuf::from("docs_out")) }
                .join(path)
                .with_extension("json");

            if let Some(parent) = parsed_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }

            fs::write(parsed_path, parsed)?;
        }
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let start = Instant::now();

    if let Err(error) = parse_path(cli.output_dir, cli.directory) {
        println!(
            "Failed to parse directory\n Reason:\n \t{}",
            error.to_string().replace("\n", "\n \t")
        );
    }

    println!("Finished in {:?}", start.duration_since(Instant::now()));
}
