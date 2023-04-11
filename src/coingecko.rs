use cli_table::{
    format::{Align, Justify},
    Color, Table,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinList {
    coins: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
    // image: String,
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
