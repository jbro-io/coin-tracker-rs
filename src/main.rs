use crate::coin::coin::{Coin, CoinList};
use cli_table::{
    format::{Align, Justify},
    Cell, CellStruct, Color, Style, Table, WithTitle,
};
use console::{style, Term};
use std::process::exit;
use std::{
    error::Error,
    io::{self, Write},
    thread,
    time::Duration,
};
use tokio::signal;

mod coin;
mod coin_table;

// impl fmt::Debug for Coin {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Point")
//             .field("id", &self.id)
//             .field("symbol", &self.symbol)
//             .field("current_price", &self.current_price)
//             .finish()
//     }
// }

async fn get_coin_data() -> Result<Vec<Coin>, Box<dyn Error>> {
    let url = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids=bitcoin,ethereum,solana,avalanche-2,algorand,render-token,synapse-2,staked-ether,msol,marinade";
    let response_body = reqwest::get(url).await?.json::<Vec<Coin>>().await?;
    Ok(response_body)
}

struct TableOption {
    cell: Box<dyn cli_table::Cell>,
    value: Box<dyn cli_table::Cell>,
}

async fn do_stuff() -> io::Result<()> {
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
                for coin in coins {
                    match coin {
                        _ => {
                            // println!("{:#?}", coin);
                            table_options.push(vec![
                                coin.name.cell(),
                                coin.symbol.to_uppercase().cell().justify(Justify::Left),
                                coin_table::get_currency_cell(coin.current_price),
                                coin_table::get_currency_cell(coin.price_change_24h),
                                coin_table::get_percentage_cell(coin.price_change_percentage_24h),
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
            }
        }

        let header_size = 4;
        let row_size = 2;
        let move_cursor_up = table_options.len() * row_size + header_size;

        if x != 0 {
            term.move_cursor_up(move_cursor_up)?;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tokio::spawn(async move {
        let term = Term::stdout();
        match signal::ctrl_c().await {
            Ok(()) => {
                term.show_cursor();
                println!("Ctrl + C Pressed!");
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
