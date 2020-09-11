//! CLI parameter parsing.

use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(author)]
pub struct Config {
    /// Do not print results to stdout.
    #[structopt(short, long)]
    pub silent: bool,

    /// Benchmark source directory.
    #[structopt(short, long, value_name = "DIRECTORY", default_value = "./benchmarks")]
    pub benchmarks: PathBuf,

    /// Number of warmup runs.
    #[structopt(long, value_name = "NUM", default_value = "1")]
    pub warmup: usize,

    /// Minimum number of bytes per benchmark iteration.
    #[structopt(long, value_name = "BYTES", default_value = "1048576")]
    pub min_bytes: usize,

    /// Maximum number of iterations per benchmark.
    #[structopt(long, value_name = "NUM")]
    pub max_runs: Option<usize>,

    /// Maximum number of seconds per benchmark.
    #[structopt(long, value_name = "SECONDS", default_value = "10")]
    pub max_secs: u64,

    /// Output results to a Gnuplot compatible DAT file.
    #[structopt(long, value_name = "FILE")]
    pub dat: Option<PathBuf>,
}
