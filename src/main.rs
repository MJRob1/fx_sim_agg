mod aggregator;
mod simulator;
use tokio_stream::StreamExt;

fn main() {
    let config = simulator::get_configs("resources/config.txt");

    fx_sim_agg::run(async {
        // async returns a future rather than blocking current thread
        // run starts a runtime and hands the future to the runtime
        // all the code - the entire program is the signature future argument of run!
        // note: everything inside the async code avoids blocking but any code outside run will
        // block on the run function returning

        // Combine all individual market data streams from each liquidity provider into a single merged stream
        // that yields values in the order they arrive from the source market data streams
        let mut merged_streams_map = simulator::start_streams(&config);

        while let Some(val) = merged_streams_map.next().await {
            // await polls the future until future returns Ready.
            // If future still pending then control is handed to the runtime
            let (key, message) = val;
            println!("key: {key}, val: {message}");
        }
    });
}
