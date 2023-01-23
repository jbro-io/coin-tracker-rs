use crate::coin_table::{get_currency_cell, get_percentage_cell};
use crate::coingecko::Coin;
use crate::config::{get_coin_position, get_coins_as_string};
use chrono::prelude::*;
use cli_table::{format::Justify, Cell, CellStruct, Style, Table, TableStruct};
use console::Term;
use std::process::exit;
use std::time::SystemTime;
use std::{
    error::Error,
    io::{self},
    thread,
    time::Duration,
};
use tokio::signal;

async fn get_coin_data() -> Result<Vec<Coin>, Box<dyn Error>> {
    let base_url = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids=";
    let url = String::from(base_url) + &get_coins_as_string();

    let response_body = reqwest::get(url).await?.json::<Vec<Coin>>().await?;
    Ok(response_body)
}

fn calc_coin_position_value(position: f64, current_price: f64) -> f64 {
    position * current_price
}

fn build_coin_rows(coins: Vec<Coin>) -> Vec<Vec<CellStruct>> {
    let mut table_options = vec![];
    let mut portfolio_value = 0.0;

    for coin in coins {
        let position = get_coin_position(&coin.id);
        let value = calc_coin_position_value(position, coin.current_price);
        portfolio_value += value;
        table_options.push(vec![
            coin.name.cell(),
            coin.symbol.to_uppercase().cell().justify(Justify::Left),
            get_currency_cell(coin.current_price),
            get_currency_cell(coin.price_change_24h),
            get_percentage_cell(coin.price_change_percentage_24h),
            coin.high_24h.cell().justify(Justify::Right),
            coin.low_24h.cell().justify(Justify::Right),
            "".cell(),
            position.to_string().cell(),
            get_currency_cell(value),
        ]);
    }

    // add total to last row of table
    table_options.push(vec![
        "".cell(),
        "".cell(),
        "".cell(),
        "".cell(),
        "".cell(),
        "".cell(),
        "".cell(),
        "".cell(),
        "".cell(),
        get_currency_cell(portfolio_value),
    ]);

    table_options
}

fn build_table_with_header(table_options: Vec<Vec<CellStruct>>) -> TableStruct {
    return table_options
        .table()
        .title(vec![
            "Name".cell().bold(true),
            "Symbol".cell().bold(true),
            "Current Price".cell().bold(true),
            "Change (24h)".cell().bold(true),
            "Change% (24h)".cell().bold(true),
            "High (24h)".cell().bold(true),
            "Low (24h)".cell().bold(true),
            "".cell().bold(true),
            "Position".cell().bold(true),
            "Position Value".cell().bold(true),
        ])
        .bold(true);
}

async fn execute() -> io::Result<()> {
    let term = Term::stdout();
    term.set_title("Crypto Tracker");
    term.clear_screen()?;
    term.hide_cursor()?;

    let mut x = 0u32;
    loop {
        let coin_list = get_coin_data().await;
        let mut table_options = vec![];
        match coin_list {
            Ok(coins) => {
                table_options = build_coin_rows(coins);

                let header_size = 4;
                let footer_size = 3;
                let row_size = 2;
                let move_cursor_up = table_options.len() * row_size + header_size + footer_size;

                if x != 0 {
                    term.move_cursor_up(move_cursor_up)?;
                }

                let table = build_table_with_header(table_options);
                let table_display = table.display().unwrap();

                match term.write_line(&format!("{}", table_display)) {
                    Err(e) => return Err(e),
                    Ok(t) => {
                        // display last updated at timestamp
                        let now = Local::now();
                        println!("Last updated: {:0}", now);
                    }
                };
            }
            _ => {
                eprintln!(
                    "Error retrieving data from CoinGecko. Restarting tracker in 30 seconds."
                );

                // restart tracker after 30 seconds
                thread::sleep(Duration::from_secs(30));
                execute();
            }
        }

        x += 1;
        thread::sleep(Duration::from_secs(60));
    }
}

pub async fn run_tracker() -> Result<(), Box<dyn Error>> {
    tokio::spawn(async move {
        let term = Term::stdout();
        match signal::ctrl_c().await {
            Ok(()) => {
                let _ = term.show_cursor();
                println!("Exiting Crypto Tracker...");
                exit(0);
            }
            Err(e) => {
                eprintln!("Error: Unable to listen for shutdown signal: {}", e);
            }
        }
    });

    match execute().await {
        Ok(t) => t,
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}
