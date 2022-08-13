use crate::coin::coin::Coin;
use crate::coin_table::{get_currency_cell, get_percentage_cell};
use crate::config::get_coins_as_string;
use cli_table::{format::Justify, Cell, Style, Table};
use console::Term;
use std::process::exit;
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

    //TODO: verify coins are in the list before api request
    let response_body = reqwest::get(url).await?.json::<Vec<Coin>>().await?;
    Ok(response_body)
}

async fn do_stuff() -> io::Result<()> {
    let term = Term::stdout();
    term.set_title("Crypto Tracker");
    term.clear_screen()?;
    term.hide_cursor()?;

    let mut message_added = false;
    let mut x = 0u32;
    loop {
        let coin_list = get_coin_data().await;
        let mut table_options = vec![];
        match coin_list {
            Ok(coins) => {
                for coin in coins {
                    match coin {
                        _ => {
                            table_options.push(vec![
                                coin.name.cell(),
                                coin.symbol.to_uppercase().cell().justify(Justify::Left),
                                get_currency_cell(coin.current_price),
                                get_currency_cell(coin.price_change_24h),
                                get_percentage_cell(coin.price_change_percentage_24h),
                                "".cell(),
                                coin.high_24h.cell().justify(Justify::Right),
                                coin.low_24h.cell().justify(Justify::Right),
                                // format!(" {}/11", style(x + 1).cyan())
                                //     .cell()
                                //     .justify(Justify::Right),
                            ]);
                        }
                    }
                }
            }
            _ => {
                println!("Im fucked!");
                message_added = true;
            }
        }

        let header_size = 4;
        let row_size = 2;
        let move_cursor_up = table_options.len() * row_size + header_size;

        if x != 0 {
            if message_added {
                term.move_cursor_up(move_cursor_up + 1)?;
                message_added = false;
            } else {
                term.move_cursor_up(move_cursor_up)?;
            }
        }

        let table = table_options
            .table()
            .title(vec![
                "Name".cell().bold(true),
                "Symbol".cell().bold(true),
                "Current Price".cell().bold(true),
                "Change (24h)".cell().bold(true),
                "Change% (24h)".cell().bold(true),
                "".cell().bold(true),
                "High (24h)".cell().bold(true),
                "Low (24h)".cell().bold(true),
                // "Age (in years)".cell().bold(true),
            ])
            .bold(true);

        let table_display = table.display().unwrap();

        match term.write_line(&format!("{}", table_display)) {
            Err(e) => return Err(e),
            Ok(t) => t,
        };

        x += 1;
        thread::sleep(Duration::from_secs(60));
    }
    // term.show_cursor()?;
    // term.clear_last_lines(1)?;
    // term.write_line("Done counting!")?;
    // writeln!(&term, "Hello World!")?;
    //
    // write!(&term, "To edit: ")?;
    // let res = term.read_line_initial_text("default")?;
    // writeln!(&term, "\n{}", res)?;
    //
    // Ok(())
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
                eprintln!("Unable to listen for shutdown signal: {}", e);
            }
        }
    });

    match do_stuff().await {
        Ok(t) => t,
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}
