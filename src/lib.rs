use core::f64;
use rand::Rng;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{error::Error, fs, future::Future};
use tokio::{runtime::Runtime, spawn, sync::mpsc::unbounded_channel, time::sleep};
use tokio_stream::{Stream, StreamMap, wrappers::UnboundedReceiverStream};

#[derive(Debug)]
pub struct Config {
    pub lp: String,
    pub fx_pair: String,
    pub buy_price: f64,
    pub spread: f64,
}

pub fn get_configs(filepath: &str) -> Vec<Config> {
    let parameters = read_config_file(filepath);
    let mut config_vector: Vec<Config> = Vec::new();
    for i in &parameters {
        println!("{i}");
        let mut fx_params = i.split(",");
        let lp = fx_params.next().unwrap_or("BOB");
        let fx_pair = fx_params.next().unwrap_or("SILLY");
        let buy_price = fx_params.next().unwrap_or("1.5552");
        let spread = fx_params.next().unwrap_or("6");
        let buy_price_fp: f64 = buy_price.trim().parse().unwrap();
        let spread_fp: f64 = spread.trim().parse().unwrap();
        let spread_dec = spread_fp / 10000.0;
        let config = Config {
            lp: String::from(lp),
            fx_pair: String::from(fx_pair),
            buy_price: buy_price_fp,
            spread: spread_dec,
        };
        config_vector.push(config);
    }

    config_vector
}

fn read_config_file(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("problem reading initial config file")
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

pub fn run<F: Future>(future: F) -> F::Output {
    let rt = Runtime::new().unwrap();
    rt.block_on(future)
}

pub fn get_marketdata(config: &Config) -> impl Stream<Item = String> {
    let (tx, rx) = unbounded_channel();

    let pip_change: f64 = 0.0004;
    let mut price = config.buy_price;
    let lp = config.lp.clone();

    spawn(async move {
        // spawn a task to handle the async sleep calls
        // async returns a future rather than blocking current thread
        // move is required to move tx into the async block so it gets ownership and
        // tx closes after last message is sent
        for number in 1..100 {
            let pip_change: f64 = rand::random_range(1.0..5.0) / 10000.0;
            if rand::rng().random_bool(0.5) {
                price = ((price + pip_change) * 10000.0).round() / 10000.0; // Need to change for USD/JPY
            } else {
                price = ((price - pip_change) * 10000.0).round() / 10000.0; // Need to change for USD/JPY
            };
            println!("#{} {} new price before send is {}", number, lp, price);
            let message = price.to_string();

            sleep(Duration::from_millis(50)).await;
            // await polls the future until future returns Ready.
            // If future still pending then control is handed to the runtime

            if let Err(send_error) = tx.send(format!("{message}")) {
                //note format must expand to borrow message and hence you can use it again in the eprintln below
                eprintln!("Could not send message {message}: {send_error}");
                break;
            };
        }
    });

    UnboundedReceiverStream::new(rx)
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

impl FxMarketData {
    //   fn update_to_next_value(&mut self) {
    // Calculate random price change up to a maximum of 5 pips
    // Randomly add or subtract this price change to calculate new fx rate

    //       let random_pip_change: f64 = rand::random_range(1.0..f64::from(self.max_pips_change));
    //       let random_price_change = random_pip_change / 10000.0; // Need to change for USD/JPY
    // round this to 4 decimal places - seems this is the only way to do it in rust?  Need to change to 2 dec places for USD/JPY
    //       let rounded_price_change = (random_price_change * 10000.0).round() / 10000.0;

    //       if rand::rng().random_bool(0.5) {
    //           self.buy_prices[0] =
    //              ((self.buy_prices[0] + rounded_price_change) * 10000.0).round() / 10000.0; // Need to change for USD/JPY
    //      } else {
    //          self.buy_prices[0] =
    //              ((self.buy_prices[0] - rounded_price_change) * 10000.0).round() / 10000.0; // Need to change for USD/JPY
    //      };

    //      println!(
    //         "{} {} buy price is {}",
    //          self.liquidity_provider, self.instrument, self.buy_prices[0]
    //     );

    // need to update all other values too!
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
//pub fn generate(fx_data: &mut FxMarketData) -> Result<(), Box<dyn Error>> {
//    for number in 1..4 {
//        fx_data.update_to_next_value();
//    }
//
//    Ok(())
//  }
//}

pub fn start_streams(config: &Vec<Config>) -> StreamMap<i32, impl Stream<Item = String>> {
    let mut index = 0;
    let mut map = StreamMap::new();
    for i in config {
        let marketdata = get_marketdata(i);

        map.insert(index, marketdata);
        index += 1;
    }
    map
}
