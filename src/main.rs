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

fn default_out() -> String {
    "index.csv".to_string()
}

#[derive(Subcommand)]
enum Commands {
    /// Index your code
    Index {
        /// code directory
        dir: Option<PathBuf>,
        /// index output
        #[arg(default_value_t = default_out())]
        out: String,
    },
}

fn search() -> Result<(), Error> {
    println!("search");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
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
        Some(Commands::Index { dir, out }) => {
            let csv = PathBuf::from(out);
            match &dir {
                Some(dir) => {
                    index::index_all_files(&dir, &csv).await?;
                }
                None => {
                    let dir = env::current_dir()?;
                    index::index_all_files(&dir, &csv).await?;
                }
            }
        }
        None => {
            search()?;
        }
    }
    Ok(())
}
