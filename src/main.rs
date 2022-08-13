use crate::config::{add_coin, init, list_all_coins, remove_coin};
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
    command: Option<Commands>,
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
        position: Option<f64>,
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
        Some(Commands::Init) => {
            init();
        }
        Some(Commands::Run) => {
            tracker::run_tracker().await?;
        }
        Some(Commands::Add { coin_id, position }) => {
            println!("Adding coin: {:?} {:?}", coin_id, position);
            add_coin(coin_id, position);
        }
        Some(Commands::Remove { coin_id }) => {
            println!("Removing coin: {:?}", coin_id);
            remove_coin(coin_id);
        }
        Some(Commands::List) => {
            println!("Listing all the coins...");
            list_all_coins();
        }
        None => tracker::run_tracker().await?,
    }

    Ok(())
}
