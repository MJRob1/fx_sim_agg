use crate::simulator::Config;
use core::f64;
use std::cmp::Ordering;
#[derive(Debug)]
pub struct FxBookEntry {
    pub liquidity_provider: String,
    pub volume: i32,
    pub price: f64,
    pub side: String,
}

impl PartialEq for FxBookEntry {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl PartialOrd for FxBookEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.price.partial_cmp(&other.price)
    }
}
#[derive(Debug)]
pub struct FxBook {
    pub currency_pair: String,
    pub buy_book: Vec<FxBookEntry>,
    pub sell_book: Vec<FxBookEntry>,
}

impl FxBook {
    pub fn update(&mut self, market_data: String) {
        // Need to make this fn return a result and check for that in main
        //println!("in update_fxbook: market_data is {market_data}");
        add_market_data(self, market_data);

        sort_books(self);
    }
}

fn sort_books(fx_book: &mut FxBook) {
    fx_book
        .buy_book
        .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

    fx_book
        .sell_book
        .sort_by(|a, b| match a.price.partial_cmp(&b.price).unwrap() {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        });
}

fn add_market_data(fx_book: &mut FxBook, market_data: String) {
    // need to add return value - Result?
    let mut market_data_params = market_data.split("|");
    let liquidity_provider = market_data_params.next().unwrap_or("ERROR");
    let currency_pair = market_data_params.next().unwrap_or("ERROR");

    let one_mill_buy_price: f64 = extract_value(market_data_params.next(), "-9.9999");
    let one_mill_sell_price: f64 = extract_value(market_data_params.next(), "-9.9999");
    let three_mill_buy_price: f64 = extract_value(market_data_params.next(), "-9.9999");
    let three_mill_sell_price: f64 = extract_value(market_data_params.next(), "-9.9999");
    let five_mill_buy_price: f64 = extract_value(market_data_params.next(), "-9.9999");
    let five_mill_sell_price: f64 = extract_value(market_data_params.next(), "-9.9999");
    let timestamp: u128 = market_data_params
        .next()
        .unwrap_or("1751724622274219277")
        .trim()
        .parse()
        .unwrap();

    add_book_entry(fx_book, liquidity_provider, 1, one_mill_buy_price, "Buy");
    add_book_entry(fx_book, liquidity_provider, 3, three_mill_buy_price, "Buy");
    add_book_entry(fx_book, liquidity_provider, 5, five_mill_buy_price, "Buy");
    add_book_entry(fx_book, liquidity_provider, 1, one_mill_sell_price, "Sell");
    add_book_entry(
        fx_book,
        liquidity_provider,
        3,
        three_mill_sell_price,
        "Sell",
    );
    add_book_entry(fx_book, liquidity_provider, 5, five_mill_sell_price, "Sell");
}

fn add_book_entry(
    fx_book: &mut FxBook,
    liquidity_provider: &str,
    volume: i32,
    price: f64,
    side: &str,
) {
    // need to add return value - Result?
    let new_book_entry = FxBookEntry {
        liquidity_provider: String::from(liquidity_provider),
        volume,
        price,
        side: String::from(side),
    };

    if side == "Buy" {
        fx_book.buy_book.push(new_book_entry);
    } else {
        fx_book.sell_book.push(new_book_entry);
    }
}

pub fn extract_value(value: Option<&str>, default_value: &str) -> f64 {
    value.unwrap_or(default_value).trim().parse().unwrap()
}

pub fn new(config: &Vec<Config>) -> FxBook {
    let currency_pair = config[0].currency_pair.clone();
    let mut buy_book: Vec<FxBookEntry> = Vec::new();
    let mut sell_book: Vec<FxBookEntry> = Vec::new();

    FxBook {
        currency_pair: currency_pair,
        buy_book: buy_book,
        sell_book: sell_book,
    }
}
