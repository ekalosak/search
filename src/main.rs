use std::env;
use std::io::Error;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod index;

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
                    index::index_all_files(&dir)?;
                }
                None => {
                    let dir = env::current_dir()?;
                    index::index_all_files(&dir)?;
                }
            }
        }
        None => {
            search()?;
        }
    }
    Ok(())
}
