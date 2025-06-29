use core::f64;
use rand::Rng;
use std::fs;
use std::time::Duration;
use tokio::{spawn, sync::mpsc::unbounded_channel, time::sleep};
use tokio_stream::{Stream, StreamMap, wrappers::UnboundedReceiverStream};

#[derive(Debug)]
pub struct Config {
    pub lp: String,
    pub fx_pair: String,
    pub buy_price: f64,
    pub spread: f64,
    pub three_mill_markup: f64,
    pub five_mill_markup: f64,
}

pub fn get_configs(filepath: &str) -> Vec<Config> {
    let parameters = read_config_file(filepath);
    let mut config_vector: Vec<Config> = Vec::new();
    let mut index = 0;
    for i in &parameters {
        println!("{i}");
        // ignore header line in config file
        if index > 0 {
            let mut fx_params = i.split(",");
            let lp = fx_params.next().unwrap_or("CITI");
            let fx_pair = fx_params.next().unwrap_or("USD/EUR");
            let buy_price = fx_params.next().unwrap_or("1.5552").trim().parse().unwrap();
            let spread = convert_pips(fx_params.next().unwrap_or("6"));
            let three_mill_markup = convert_pips(fx_params.next().unwrap_or(".25"));
            let five_mill_markup = convert_pips(fx_params.next().unwrap_or(".5"));
            let config = Config {
                lp: String::from(lp),
                fx_pair: String::from(fx_pair),
                buy_price,
                spread,
                three_mill_markup,
                five_mill_markup,
            };
            println!("Config is : {config:?}");
            config_vector.push(config);
        }
        index += 1;
    }

    config_vector
}

fn convert_pips(value: &str) -> f64 {
    let value_fp: f64 = value.trim().parse().unwrap();
    value_fp / 10000.0
}

fn read_config_file(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("problem reading initial config file")
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

pub fn get_marketdata(config: &Config) -> impl Stream<Item = String> {
    // For this liqudity provider in config, create the new market data values
    // and send them asynchronously (don't block and wait) every 50 (currently) milliseconds
    let (tx, rx) = unbounded_channel();

    let pip_change: f64 = 0.0004;
    let mut buy_price = config.buy_price;
    let spread = config.spread;
    let three_mill_markup = config.three_mill_markup;
    let five_mill_markup = config.five_mill_markup;
    let lp = config.lp.clone(); // NEED TO LOOK AT THIS

    spawn(async move {
        // spawn a task to handle the async sleep calls
        // async returns a future rather than blocking current thread
        // move is required to move tx into the async block so it gets ownership and
        // tx closes after last message is sent
        for number in 1..4 {
            // randomly determine whether this is a price rise or fall
            let pip_change: f64 = rand::random_range(1.0..5.0) / 10000.0;
            if rand::rng().random_bool(0.5) {
                buy_price = ((buy_price + pip_change) * 10000.0).round() / 10000.0; // Need to change for USD/JPY
            } else {
                buy_price = ((buy_price - pip_change) * 10000.0).round() / 10000.0; // Need to change for USD/JPY
            };

            let sell_price = ((buy_price + spread) * 10000.0).round() / 10000.0;
            let three_mill_buy_price =
                ((buy_price + three_mill_markup) * 10000.0).round() / 10000.0;
            let three_mill_sell_price =
                ((sell_price - three_mill_markup) * 10000.0).round() / 10000.0;
            let five_mill_buy_price = ((buy_price + five_mill_markup) * 10000.0).round() / 10000.0;
            let five_mill_sell_price =
                ((sell_price - five_mill_markup) * 10000.0).round() / 10000.0;
            let message = buy_price.to_string();
            println!(
                "#{} {} new prices before send are buy {} sell {}, 3M buy {}, 3M sell {}, 5M buy {}, 5M sell {}",
                number,
                lp,
                buy_price,
                sell_price,
                three_mill_buy_price,
                three_mill_sell_price,
                five_mill_buy_price,
                five_mill_sell_price
            );

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

pub fn start_streams(config: &Vec<Config>) -> StreamMap<i32, impl Stream<Item = String>> {
    let mut index = 0;
    let mut map = StreamMap::new();
    // start a market data simulated stream for each config (lp) value
    // Combine all individual market data streams from each liquidity provider into a single merged stream map
    for i in config {
        let marketdata = get_marketdata(i);

        map.insert(index, marketdata);
        index += 1;
    }
    map
}
