use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use std::path::Path;
use tokio::runtime::Runtime;

pub fn run<F: Future>(future: F) -> F::Output {
    let rt = Runtime::new().unwrap();
    rt.block_on(future)
}

pub fn create_log_file(filepath: &str) -> BufWriter<File> {
    let path = Path::new(filepath);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    BufWriter::new(file)
}

pub fn write_to_fix_log(writer: &mut BufWriter<File>, market_data: &String) {
    match writeln!(writer, "{}", market_data) {
        Ok(_file) => (),
        Err(error) => eprintln!("Problem writing to log file, {}", error),
    }
}
