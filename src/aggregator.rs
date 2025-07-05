use core::f64;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct BookEntrySide {
    liquidity_provider: Vec<(String, i32)>,
    price: f64,
    volume: i32,
    side: String,
}

#[derive(Debug)]
pub struct BookEntry {
    liquidity_provider: String,
    buy_vol_prices: [f64; 3],
    sell_vol_prices: [f64; 3],
}

pub struct FxBook {
    currency_pair: String,
    buy_book: Vec<BookEntrySide>,
    sell_book: Vec<BookEntrySide>,
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

pub fn update_fxbook(market_data: String) {
    //println!("in update_fxbook: market_data is {market_data}");
    let book_entry = extract_market_data(market_data);
}

fn extract_market_data(market_data: String) -> BookEntry {
    let mut market_data_params = market_data.split("|");
    let liquidity_provider = market_data_params.next().unwrap_or("CITI");
    let currency_pair = market_data_params.next().unwrap_or("USD/EUR");
    let one_mill_buy_price: f64 = market_data_params
        .next()
        .unwrap_or("1.5552")
        .trim()
        .parse()
        .unwrap();
    let one_mill_sell_price: f64 = market_data_params
        .next()
        .unwrap_or("1.5558")
        .trim()
        .parse()
        .unwrap();
    let three_mill_buy_price: f64 = market_data_params
        .next()
        .unwrap_or("1.5552")
        .trim()
        .parse()
        .unwrap();
    let three_mill_sell_price: f64 = market_data_params
        .next()
        .unwrap_or("1.5558")
        .trim()
        .parse()
        .unwrap();
    let five_mill_buy_price: f64 = market_data_params
        .next()
        .unwrap_or("1.5552")
        .trim()
        .parse()
        .unwrap();
    let five_mill_sell_price: f64 = market_data_params
        .next()
        .unwrap_or("1.5558")
        .trim()
        .parse()
        .unwrap();
    let timestamp: u128 = market_data_params
        .next()
        .unwrap_or("1.5558")
        .trim()
        .parse()
        .unwrap();

    //let marketdata = format!(
    //    "{} | {} | {} | {} | {} | {} | {} | {} | {}",
    //    liquidity_provider,
    //    currency_pair,
    //    one_mill_buy_price,
    //    one_mill_sell_price,
    //    three_mill_buy_price,
    //    three_mill_sell_price,
    //    five_mill_buy_price,
    //    five_mill_sell_price,
    //    timestamp
    //);

    //println!("in aggregator: {marketdata}");

    BookEntry {
        liquidity_provider: liquidity_provider.to_string(),
        buy_vol_prices: [
            one_mill_buy_price,
            three_mill_buy_price,
            five_mill_buy_price,
        ],
        sell_vol_prices: [
            one_mill_sell_price,
            three_mill_sell_price,
            five_mill_sell_price,
        ],
    }

    //  println!("Book entry is : {book_entry:?}");
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
