use std::{env, fs};
use std::fs::File;
use std::io::{Read, Error, ErrorKind};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Index your code
    Index {
        /// code directory
        dir: Option<PathBuf>,
    },
}

fn search() -> Result<(), Error> {
    println!("search");
    Ok(())
}

fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_vector(file: PathBuf) -> Result<(), Error> {
    let fs = file.to_str().ok_or(Error::new(ErrorKind::InvalidInput, "Invalid file path"))?;
    let fc = read_file(fs)?;
    println!("{:?}", fc);
    Ok(())
}

fn index(dir: &PathBuf) -> Result<(), Error> {
    println!("Indexing files in dir: {:?}", *dir);
    for ent in fs::read_dir(dir)? {
        let ent = ent?;
        let path = ent.path();
        let metadata = fs::metadata(&path)?;
        println!("{:?}", path);
        if metadata.is_file() {
            let vec = get_vector(path)?;
            println!("Got vec: {:?}", vec);
            println!("");
        } else {
            index(&path)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Index { dir }) => {
            match &dir {
                Some(dir) => {
                    index(&dir)?;
                }
                None => {
                    let dir = env::current_dir()?;
                    index(&dir)?;
                }
            }
        }
        None => {
            search()?;
        }
    }
    Ok(())
}
