pub mod coin {
    use cli_table::{
        format::{Align, Justify},
        Color, Table,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CoinList {
        coins: Vec<Coin>,
    }

    #[derive(Serialize, Deserialize, Table, Debug)]
    pub struct Coin {
        #[table(
            title = "ID",
            justify = "Justify::Right",
            align = "Align::Top",
            color = "Color::Green",
            bold
        )]
        pub id: String,
        #[table(title = "Symbol")]
        pub symbol: String,
        pub name: String,
        // image: String,
        #[table(title = "Current Price")]
        pub current_price: f64,
        // market_cap: u64,
        // market_cap_rank: u64,
        // fully_diluted_valuation: u64,
        // total_volume: u64,
        pub high_24h: f64,
        pub low_24h: f64,
        pub price_change_24h: f64,
        pub price_change_percentage_24h: f64,
        // market_cap_change_24h: i64,
        // market_cap_change_percentage_24h: i64,
        // circulating_supply: i64,
        // total_supply: i64,
        // max_supply: i64,
        // ath: i64,
        // ath_change_percentage: i64,
        // ath_date: String,
        // atl: i64,
        // atl_change_percentage: i64,
        // atl_date: String,
        // last_updated: String,
    }

    // async fn get_coin_data() -> Result<Vec<Coin>, Box<dyn Error>> {
    //     let url = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids=bitcoin,ethereum,solana,avalanche-2,algorand,render-token,synapse-2,staked-ether,msol,marinade";
    //     let response_body = reqwest::get(url).await?.json::<Vec<Coin>>().await?;
    //     // println!("Raw: {:#?}", response_body);
    //
    //     // Ok(())
    //     Ok(response_body)
    // }
}
