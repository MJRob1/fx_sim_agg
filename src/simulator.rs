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
