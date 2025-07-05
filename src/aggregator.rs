use crate::simulator::Config;
use core::f64;
#[derive(Debug)]
pub struct FxBookEntry {
    liquidity_provider: String,
    volume: i32,
    price: f64,
    side: String,
}

pub struct FxBook {
    currency_pair: String,
    buy_book: Vec<FxBookEntry>,
    sell_book: Vec<FxBookEntry>,
}

pub fn update_fxbook(market_data: String, fx_book: &FxBook) {
    //println!("in update_fxbook: market_data is {market_data}");
    let fx_book_entries = extract_market_data(market_data);
    add_book_entry(fx_book_entries, fx_book);
}

fn add_book_entry(fx_book_entries: Vec<FxBookEntry>, fx_book: &FxBook) {
    // println!("Book entries are: {fx_book_entries:?}");
    // fx_book_entries are the 3 buy book (1M, 3M, 5M - indexes 0,1,2)
    // and 3 sell book (1M, 3M, 5M - indexes 0,1,2) entries from the current market data input string
    if fx_book.buy_book.len() == 0 {
        // this is first entry into book
        println!("fxbook is empty");
        //let fx_book.buy_book[0] = fx_book_entries[0];
        //= FxBookEntrySide {
        //     liquidity_provider: book_entry.liquidity_provider.push((book_entry.liquidity_provider, 1)),
    } else {
        println!("fxbook length is {}", fx_book.buy_book.len());
    }
}

fn extract_market_data(market_data: String) -> Vec<FxBookEntry> {
    let mut market_data_params = market_data.split("|");
    let liquidity_provider = market_data_params.next().unwrap_or("CITI");
    let currency_pair = market_data_params.next().unwrap_or("USD/EUR");

    let one_mill_buy_price: f64 = extract_value(market_data_params.next(), "1.5552");
    let one_mill_sell_price: f64 = extract_value(market_data_params.next(), "1.5558");
    let three_mill_buy_price: f64 = extract_value(market_data_params.next(), "1.5552");
    let three_mill_sell_price: f64 = extract_value(market_data_params.next(), "1.5558");
    let five_mill_buy_price: f64 = extract_value(market_data_params.next(), "1.5552");
    let five_mill_sell_price: f64 = extract_value(market_data_params.next(), "1.5558");
    let timestamp: u128 = market_data_params
        .next()
        .unwrap_or("1751724622274219277")
        .trim()
        .parse()
        .unwrap();

    let price_volumes = [
        (1, one_mill_buy_price),
        (3, three_mill_buy_price),
        (5, five_mill_buy_price),
        (1, one_mill_sell_price),
        (3, three_mill_sell_price),
        (5, five_mill_buy_price),
    ];

    let mut fx_book_entries: Vec<FxBookEntry> = Vec::new();
    for i in 0..6 {
        let mut side = String::from("Buy");
        if i > 2 {
            side = String::from("Sell");
        }
        let fx_book_entry = FxBookEntry {
            liquidity_provider: String::from(liquidity_provider),
            volume: price_volumes[i].0,
            price: price_volumes[i].1,
            side,
        };
        fx_book_entries.push(fx_book_entry);
    }

    //println!("Book entries are: {fx_book_entries:?}");

    fx_book_entries
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
