use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::io;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug)]
struct CoinConfig {
    coin_id: String,
    position: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    coins: Vec<CoinConfig>,
}

fn create_config_file(output_path: PathBuf) {
    let default_config = Config { coins: vec![] };

    println!("Creating config file at: {:?}", output_path);

    let write_result = std::fs::write(
        output_path,
        serde_json::to_string_pretty(&default_config).unwrap(),
    );

    match write_result {
        Ok(_) => println!("Config successfully created."),
        Err(error) => println!("Problem creating config file: {:?}", error),
    }
}

/// Creates a json config file for the tracker if one does not already exist
pub fn init() {
    let mut output_path = PathBuf::new();
    //TODO: update to home path
    // output_path.push(dirs::home_dir().unwrap());
    // output_path.push(".config");
    // output_path.push("cointracker");
    output_path.push("./cointracker_config.json");

    if output_path.exists() {
        println!("Config file already exists. Do you want to overwrite the existing file? (y/n)");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim() {
                "y" => {
                    println!("Creating new config file...");
                    create_config_file(output_path);
                }
                _ => {}
            },
            Err(error) => {
                panic!("Unable to read answer: {:?}", error);
            }
        }
    } else {
        create_config_file(output_path)
    }
}
