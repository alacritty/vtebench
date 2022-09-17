//! CLI argument parsing.

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author)]
pub struct Config {
    /// Do not print results to stdout.
    #[clap(short, long)]
    pub silent: bool,

    /// Benchmark source directories.
    #[clap(short, long, value_name = "DIRECTORY", default_value = "./benchmarks")]
    pub benchmarks: Vec<PathBuf>,

    /// Number of warmup iterations.
    #[clap(long, value_name = "NUM", default_value = "1")]
    pub warmup: usize,

    /// Minimum number of bytes per benchmark sample.
    #[clap(long, value_name = "BYTES", default_value = "1048576")]
    pub min_bytes: usize,

    /// Maximum number of samples per benchmark.
    #[clap(long, value_name = "NUM")]
    pub max_samples: Option<usize>,

    /// Maximum number of seconds per benchmark.
    #[clap(long, value_name = "SECONDS", default_value = "10")]
    pub max_secs: u64,

    /// Output results to a Gnuplot compatible DAT file.
    #[clap(long, value_name = "FILE")]
    pub dat: Option<PathBuf>,
}
