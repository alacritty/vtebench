#![deny(clippy::all)]

mod bench;
mod cli;
mod format;

use clap::Parser;

use crate::bench::Results;
use crate::cli::Config;
use crate::format::{DatFormat, Format, StdoutFormat};

fn main() {
    // Parse CLI parameters.
    let config = Config::parse();

    // Find all available benchmarks.
    let mut loaders = bench::find_benchmarks(&config.benchmarks);

    // Run all benchmarks one at a time.
    let results: Vec<Results> = loaders
        .drain(..)
        .filter_map(|loader| loader.load(config.min_bytes).ok())
        .map(|bench| bench.run(config.warmup, config.max_secs, config.max_samples))
        .collect();

    // Output results in various formats based on CLI config.

    if !config.silent {
        StdoutFormat.format(&results);
    }

    if let Some(path) = config.dat {
        DatFormat::new(path).format(&results);
    }
}
