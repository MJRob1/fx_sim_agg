//! # FX Simulator and Aggregator - fx_sim_agg
//!
//! `fx_sim_agg` simulates FX market data streams and aggregates them into a real-time book of buys and sells.
//!
//! - `main.rs` combines all the individual asynchronous market data streams from each liquidity provider into a single merged stream
//! that yields values in the order they arrive from the source market data streams
//! - `simulator.rs` generates simulated FX market data and sends the data as asynchronous market data streams
//! - `aggregator.rs` updates and aggregates the asynchronous data streams into a real-time FX book of buys and sells
//! - `lib.rs` various utilities used by the other modules
mod aggregator;
mod simulator;
use std::process::exit;
//use log::{debug, error, info, trace, warn};
use log::error;
use log4rs;
use tokio_stream::StreamExt;

fn main() {
    // start log4rs logging framework
    if let Err(e) = log4rs::init_file("logging_config.yaml", Default::default()) {
        eprintln!("error initialising log4rs - {e}");
        exit(1);
    }

    // Create "FIX" log file
    let mut writer = match fx_sim_agg::create_log_file("logs/fix.log") {
        Ok(writer) => writer,
        Err(e) => {
            error!("problem creating log file - {e}");
            exit(1);
        }
    };

    // read config file to get configs for each liquidity provider source
    let mut configs: Vec<simulator::Config> = Vec::new();
    if let Err(e) = simulator::get_configs(&mut configs) {
        error!("config input file not processed - {e}");
        exit(1);
    }

    // Create aggregated FX Book
    let mut fx_book = aggregator::new(&configs);

    fx_sim_agg::run(async {
        /*  async returns a future rather than blocking current thread
        run() starts a runtime and hands the future to the runtime all the code - the entire program
        is the signature future argument of run! Note: everything inside the async code avoids blocking
        but any code outside run will block on the run function returning */

        // Combine all individual market data streams from each liquidity provider into a single merged stream
        // that yields values in the order they arrive from the source market data streams
        let mut merged_streams_map = simulator::start_streams(&configs);

        while let Some(val) = merged_streams_map.next().await {
            // await polls the future until future returns Ready.
            // If future still pending then control is handed to the runtime
            let (_key, market_data) = val;

            // write market data to a "FIX" log
            if let Err(e) = fx_sim_agg::write_to_fix_log(&mut writer, &market_data) {
                error!("problem writing to FIX log - {e}");
            }

            // Update the Fx Book with the new market data
            if let Err(e) = fx_book.update(market_data) {
                error!("market data not processed - {e}");
            } else {
                // currently working on real-time GUI rather than print!!
                aggregator::print_fxbook_as_ladder(&mut fx_book);
            }
        }
    });
}
