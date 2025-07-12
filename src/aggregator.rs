use crate::simulator::Config;
use core::f64;
use std::cmp::Ordering;

enum Aggregated<T> {
    Added,
    ToAdd(T),
}

#[derive(Debug)]
pub struct FxAggBookEntry {
    pub lp_vol: Vec<(String, i32)>,
    pub volume: i32,
    pub price: f64,
    pub side: String,
}

impl PartialEq for FxAggBookEntry {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl PartialOrd for FxAggBookEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.price.partial_cmp(&other.price)
    }
}
#[derive(Debug)]
pub struct FxBook {
    pub currency_pair: String,
    pub buy_book: Vec<FxAggBookEntry>,
    pub sell_book: Vec<FxAggBookEntry>,
}

impl FxBook {
    pub fn update(&mut self, market_data: String) {
        // Need to make this fn return a result and check for that in main
        add_market_data(self, market_data);
        //check_remove_entry(self);

        sort_books(self);
    }
}

fn check_remove_entry(fx_book: &mut FxBook) {
    fx_book.buy_book.remove(0);
}

fn sort_books(fx_book: &mut FxBook) {
    fx_book
        .buy_book
        .sort_by(|a, b| match a.price.partial_cmp(&b.price).unwrap() {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        });

    fx_book
        .sell_book
        .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
}

fn add_market_data(fx_book: &mut FxBook, market_data: String) {
    // need to add return value - Result?
    println!("market data is {market_data}");
    let mut vol_prices_vec: Vec<(i32, f64, String)> = Vec::new();

    let mut market_data_params = market_data.split("|");
    let liquidity_provider = market_data_params.next().unwrap_or("ERROR");
    let _currency_pair = market_data_params.next().unwrap_or("ERROR");

    let one_mill_buy_price: f64 = extract_value(market_data_params.next(), "-9.9999");
    vol_prices_vec.push((1, one_mill_buy_price, String::from("Buy")));
    let one_mill_sell_price: f64 = extract_value(market_data_params.next(), "-9.9999");
    vol_prices_vec.push((1, one_mill_sell_price, String::from("Sell")));
    let three_mill_buy_price: f64 = extract_value(market_data_params.next(), "-9.9999");
    vol_prices_vec.push((3, three_mill_buy_price, String::from("Buy")));
    let three_mill_sell_price: f64 = extract_value(market_data_params.next(), "-9.9999");
    vol_prices_vec.push((3, three_mill_sell_price, String::from("Sell")));
    let five_mill_buy_price: f64 = extract_value(market_data_params.next(), "-9.9999");
    vol_prices_vec.push((5, five_mill_buy_price, String::from("Buy")));
    let five_mill_sell_price: f64 = extract_value(market_data_params.next(), "-9.9999");
    vol_prices_vec.push((5, five_mill_sell_price, String::from("Sell")));
    let _timestamp: u128 = market_data_params
        .next()
        .unwrap_or("1751724622274219277")
        .trim()
        .parse()
        .unwrap();

    let mut i = 0;
    for val in vol_prices_vec {
        //Not using returned Aggregated enum in match now
        if i % 2 == 0 {
            //remove expired quotes before adding any new quotes
            //  check_expired_quotes(fx_book, liquidity_provider, "Buy", val.0);
            add_agg_book_entry(fx_book, liquidity_provider, val.0, val.1, "Buy");
        } else {
            //  check_expired_quotes(fx_book, liquidity_provider, "Sell", val.0);
            add_agg_book_entry(fx_book, liquidity_provider, val.0, val.1, "Sell");
        }
        i += 1;
    }
}

fn add_agg_book_entry(
    fx_book: &mut FxBook,
    liquidity_provider: &str,
    volume: i32,
    price: f64,
    side: &str,
) -> Aggregated<FxAggBookEntry> {
    // need to add return value - Result?

    let mut lp_vol_vec: Vec<(String, i32)> = Vec::new();
    lp_vol_vec.push((String::from(liquidity_provider), volume));

    // if first entry then just add it to book
    // and using fact that first entry is always a Buy in current config
    if fx_book.buy_book.len() == 0 {
        let new_agg_book_entry = FxAggBookEntry {
            lp_vol: lp_vol_vec,
            volume,
            price,
            side: String::from(side),
        };
        fx_book.buy_book.push(new_agg_book_entry);
        return Aggregated::Added;
    } else {
        // first check to see if existing quotes have expired
        // and remove them

        // check_expired_quotes(fx_book, liquidity_provider, side, volume);

        let fx_book_side = get_book_side(fx_book, side);

        //search to see if current price already in aggregated book
        for entry in fx_book_side {
            if entry.price == price {
                let lp_tup = (String::from(liquidity_provider), volume);
                entry.lp_vol.push(lp_tup);
                entry.volume += volume;
                return Aggregated::Added;
            }
        }

        // this is new entry
        let new_agg_book_entry = FxAggBookEntry {
            lp_vol: lp_vol_vec,
            volume,
            price,
            side: String::from(side),
        };
        let fx_book_side = get_book_side(fx_book, side);
        fx_book_side.push(new_agg_book_entry);
        return Aggregated::Added;
        // return Aggregated::ToAdd(new_agg_book_entry);
    }
}

fn get_book_side<'a>(fx_book: &'a mut FxBook, side: &str) -> &'a mut Vec<FxAggBookEntry> {
    if String::from(side) == String::from("Buy") {
        &mut fx_book.buy_book
    } else {
        &mut fx_book.sell_book
    }
}
fn check_expired_quotes(fx_book: &mut FxBook, liquidity_provider: &str, side: &str, volume: i32) {
    let fx_book_side = get_book_side(fx_book, side);
    for entry in fx_book_side {
        println!("entry is {entry:?}");
        let lp_vol_vec = &mut entry.lp_vol;
        println!(
            "lp_vol_vec before retain {lp_vol_vec:?}, lp is {liquidity_provider}, and volume is {volume}."
        );
        lp_vol_vec.retain(|lp_vol| {
            ((lp_vol.0 != liquidity_provider)
                || ((lp_vol.0 == liquidity_provider) && (lp_vol.1 != volume)))
        });
        println!("lp_vol_vec after retain {lp_vol_vec:?}");
        // need to re-sum the total volumes here in case an expired quote has been removed
    }
    /*    let mut index = 0;
    let mut remove = false;
    let mut remove_index = 0;
    for lp_vol in lp_vol_vec {
        if (lp_vol.0 == liquidity_provider) && (lp_vol.1 == volume) {
            println!(
                "liquidity provider {} and volume {} already present",
                liquidity_provider, volume
            );
            remove = true;
            remove_index = index;
            println!("in loop: remove is {remove} and remove index is {remove_index}");
            break;
        }

        index += 1;
    }
    println!("remove is {remove} and remove index is {remove_index}");
    // lp_vol_vec.remove(index);  */
}

pub fn extract_value(value: Option<&str>, default_value: &str) -> f64 {
    value.unwrap_or(default_value).trim().parse().unwrap()
}

pub fn new(config: &Vec<Config>) -> FxBook {
    let currency_pair = config[0].currency_pair.clone();
    let mut buy_book: Vec<FxAggBookEntry> = Vec::new();
    let mut sell_book: Vec<FxAggBookEntry> = Vec::new();

    FxBook {
        currency_pair: currency_pair,
        buy_book: buy_book,
        sell_book: sell_book,
    }
}
