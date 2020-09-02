use structopt::StructOpt;

mod bench;
mod cli;
mod format;

use crate::bench::Results;
use crate::cli::Config;
use crate::format::stdout::StdoutFormat;
use crate::format::Format;

fn main() {
    // Parse CLI parameters.
    let config = Config::from_args();

    // Find all available benchmarks.
    let mut loaders = bench::find_benchmarks(&config.benchmarks);

    // Run all benchmarks one at a time.
    let results: Vec<Results> = loaders
        .drain(..)
        .filter_map(|loader| loader.load(config.min_bytes).ok())
        .map(|bench| bench.run(config.warmup, config.max_secs, config.max_runs))
        .collect();

    // Write to stdout.
    if !config.silent {
        StdoutFormat.format(&results);
    }
}
