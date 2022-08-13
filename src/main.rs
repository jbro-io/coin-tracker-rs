use crate::config::init;
use clap::{Parser, Subcommand};
use std::error::Error;

mod coin;
mod coin_table;
mod config;
mod tracker;

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the tracker and creates a json file ~/.coin_tracker
    Init,
    /// Runs the tracker
    Run,
    /// Adds a coin to the tracker list
    Add {
        /// CoinGecko coin id
        coin_id: String,
    },
    /// Removes a token from the tracker list
    Remove {
        /// CoinGecko coin id
        coin_id: String,
    },
    /// List all the current coins being tracked
    List,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match &args.command {
        Commands::Init => {
            init();
        }
        Commands::Run => {
            tracker::run_tracker().await?;
        }
        Commands::Add { coin_id } => {
            //TODO: add the coin to the tracker json file
            println!("Adding coin id: {:?}", coin_id);
        }
        Commands::Remove { coin_id } => {
            //TODO: remove the coin from the tracker json file
            println!("Removing coin id: {:?}", coin_id);
        }
        Commands::List => {
            //TODO: display all the coins being tracked
            println!("List all the coins...");
        }
    }

    Ok(())
}
