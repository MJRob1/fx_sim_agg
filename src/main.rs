use std::{pin::pin, time::Duration};
use tokio_stream::{StreamExt, StreamMap};

fn main() {
    let config = fx_sim_agg::get_configs("resources/config.txt");

    fx_sim_agg::run(async {
        // async returns a future rather than blocking current thread
        // run starts a runtime and hands the future to the runtime
        // all the code - the entire program is the signature future argument of run!
        // note: everything inside the async code avoids blocking but any code outside run will
        // block on the run function returning

        let mut map = fx_sim_agg::start_streams(&config);

        while let Some(val) = map.next().await {
            // await polls the future until future returns Ready.
            // If future still pending then control is handed to the runtime
            let (key, message) = val;
            println!("key: {key}, val: {message}");
        }
    });
}
