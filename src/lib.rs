//! # FX Simulator and Aggregator - fx_sim_agg
//!
//! `fx_sim_agg` simulates FX market data streams and aggregates them into a real-time book of buys and sells.
//!
//! - `main.rs` combines all the individual asynchronous market data streams from each liquidity provider into a single merged stream
//! that yields values in the order they arrive from the source market data streams
//! - `simulator.rs` generates simulated FX market data and sends the data as asynchronous market data streams
//! - `aggregator.rs` updates and aggregates the asynchronous data streams into a real-time FX book of buys and sells
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::prelude::*;
use std::num::ParseFloatError;
use std::num::ParseIntError;
use std::path::Path;
use tokio::runtime::Runtime;

#[derive(Debug)]
#[non_exhaustive]
pub enum AppError {
    NumParams,
    IsEmpty,
    ParseFloat(ParseFloatError),
    ParseInt(ParseIntError),
    Io(io::Error),
}

impl From<ParseFloatError> for AppError {
    fn from(error: ParseFloatError) -> Self {
        Self::ParseFloat(error)
    }
}

impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        Self::ParseInt(error)
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::IsEmpty => f.write_str("empty data field"),
            Self::NumParams => f.write_str("missing market data fields"),
            Self::ParseFloat(e) => Display::fmt(e, f),
            Self::ParseInt(e) => Display::fmt(e, f),
            Self::Io(e) => Display::fmt(e, f),
        }
    }
}

impl std::error::Error for AppError {}

pub fn run<F: Future>(future: F) -> F::Output {
    let rt = Runtime::new().unwrap();
    rt.block_on(future)
}

pub fn create_log_file(file_path: &str) -> Result<BufWriter<File>, AppError> {
    let path = Path::new(file_path);

    // Open a file in write-only mode, returns `io::Result<File>`
    let file = File::create(&path)?;

    Ok(BufWriter::new(file))
}

pub fn write_to_fix_log(
    writer: &mut BufWriter<File>,
    market_data: &String,
) -> Result<(), AppError> {
    writeln!(writer, "{}", market_data)?;
    Ok(())
}

pub fn get_params(data: &str, number: usize) -> Result<std::str::Split<'_, &str>, AppError> {
    let value = data.split("|");
    if value.clone().count() < number {
        return Err(AppError::NumParams);
    } else {
        Ok(data.split("|"))
    }
}

pub fn get_str_field(field: Option<&str>) -> Result<&str, AppError> {
    let value = field.unwrap_or("");
    if value.trim().is_empty() {
        return Err(AppError::IsEmpty);
    } else {
        Ok(value.trim())
    }
}
