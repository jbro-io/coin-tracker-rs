use clap::{Parser, Subcommand};
use std::error::Error;

mod coin;
mod coin_table;
mod tracker;

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match &args.command {
        Commands::Run => {
            tracker::run_tracker().await?;
        }
        Commands::Add { coin_id } => {
            println!("Adding coin id: {:?}", coin_id);
        }
        Commands::Remove { coin_id } => {
            println!("Removing coin id: {:?}", coin_id);
        }
    }

    Ok(())
}
