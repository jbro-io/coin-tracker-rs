use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::{fs, io};

#[derive(Deserialize, Serialize, Debug)]
struct CoinConfig {
    coin_id: String,
    position: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    coins: Vec<CoinConfig>,
}

fn get_config_folder_path() -> PathBuf {
    let mut output_path = PathBuf::new();
    output_path.push(home_dir().unwrap());
    output_path.push(".config");
    output_path.push("cointracker");
    output_path
}

fn get_config_file_path() -> PathBuf {
    let mut path = get_config_folder_path();
    path.push("cointracker_config.json");
    path
}

fn parse_config_file() -> Result<Config, Box<dyn Error>> {
    let config_path = get_config_file_path();
    let file = File::open(config_path)?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}

fn create_config_file(output_path: PathBuf) {
    let default_config = Config { coins: vec![] };

    // create config directory structure if it doesnt already exist
    let folder = get_config_folder_path();
    fs::create_dir_all(folder).expect("Error creating config directories");

    let write_result = std::fs::write(
        output_path,
        serde_json::to_string_pretty(&default_config).unwrap(),
    );

    match write_result {
        Ok(_) => println!("Config successfully created!"),
        Err(error) => println!("Problem creating config file: {:?}", error),
    }
}

fn update_config_file(config: Config) {
    let output_path = get_config_file_path();
    let write_result = std::fs::write(output_path, serde_json::to_string_pretty(&config).unwrap());

    match write_result {
        Ok(_) => println!("Config successfully updated!"),
        Err(error) => println!("Problem creating config file: {:?}", error),
    }
}

/// Creates a json config file for the tracker if one does not already exist
pub fn init() {
    let output_path = get_config_file_path();
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

fn does_coin_exist(coin_id: &String, coins: &Vec<CoinConfig>) -> bool {
    let mut coin_exists = false;
    for index in 0..coins.len() {
        if &coins[index].coin_id == coin_id {
            coin_exists = true;
            break;
        }
    }
    coin_exists
}

/// Adds a coin to the tracker if it has not already been added
pub fn add_coin(coin_id: &String, position: &Option<f64>) {
    let mut config = parse_config_file().expect("Error reading config file.");
    let coin_exists = does_coin_exist(coin_id, &config.coins);

    if !coin_exists {
        let mut new_coin = CoinConfig {
            coin_id: String::from(coin_id),
            position: 0.0,
        };

        match position {
            Some(_) => new_coin.position = position.unwrap(),
            None => {}
        }

        config.coins.push(new_coin);
        update_config_file(config);
    } else {
        println!("Coin already added to tracker.");
    }
}

/// Removes a coin from the tracker if it exists
pub fn remove_coin(coin_id: &String) {
    let mut config = parse_config_file().expect("Error reading config file.");
    let coin_exists = does_coin_exist(coin_id, &config.coins);

    if coin_exists {
        for index in 0..config.coins.len() {
            if &config.coins[index].coin_id == coin_id {
                config.coins.remove(index);
                break;
            }
        }

        update_config_file(config);
    } else {
        println!("Coin ({:?}) is not currently being tracked", coin_id)
    }
}

/// Lists all the coins in the config file
pub fn list_all_coins() {
    let config = parse_config_file().expect("Error reading config file.");

    for index in 0..config.coins.len() {
        let coin = &config.coins[index];
        println!("{:?}", &coin);
    }
}

/// Returns all the coins in the config file as a comma separated string
pub fn get_coins_as_string() -> String {
    let config = parse_config_file().expect("Error reading config file.");
    let mut coin_string = String::new();

    for index in 0..config.coins.len() {
        coin_string = coin_string + &config.coins[index].coin_id + ",";
    }
    // remove trailing comma
    coin_string.pop();
    coin_string
}

pub fn get_coin_position(coin_id: &String) -> f64 {
    let config = parse_config_file().expect("Error reading config file.");

    for index in 0..config.coins.len() {
        let coin = &config.coins[index];
        if coin_id == &coin.coin_id {
            return coin.position;
        }
    }

    0.0
}
