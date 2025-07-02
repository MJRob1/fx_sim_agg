use core::f64;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct BookEntry {
    liquidity_provider: String,
    price: f64,
    volume: i32,
    side: String,
}

pub struct FxBook {
    currency_pair: String,
    buy_book: Vec<BookEntry>,
    sell_book: Vec<BookEntry>,
}
#[derive(Debug)]
pub struct FxMarketData {
    liquidity_provider: String,
    instrument: String,
    buy_prices: [f64; 3],
    sell_prices: [f64; 3],
    volumes: [i32; 3],
    timestamp: u128,
    spread: i32,
    max_pips_change: i32,
}

pub fn new(lp: String, instrument: String, price: f64, spread: i32) -> FxMarketData {
    let liquidity_provider = lp;
    let instrument = instrument;
    let buy_prices = [price, price - 0.0003, price - 0.0005]; // need to change these as from spread and change for USD/JAP
    let fp_spread = f64::from(spread) / 10000.0; // Need to change for USD/JPY
    let sell_prices = [price + fp_spread, price + 0.0004, price + 0.0002]; // need to change these as from spread and change for USD/JPY
    let volumes = [1, 3, 5];
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let spread = spread;
    let max_pips_change = 5;

    println!(
        "{} Initial {} buy price is {}",
        liquidity_provider, instrument, buy_prices[0]
    );

    FxMarketData {
        liquidity_provider,
        instrument,
        buy_prices,
        sell_prices,
        volumes,
        timestamp,
        spread,
        max_pips_change,
    }
}
