//! Output benchmark results to stdout.

use crate::bench::Results;
use crate::format::Format;

/// Text stdout format.
pub struct StdoutFormat;

impl Format for StdoutFormat {
    fn format(&self, results: &[Results]) {
        println!("Results:");
        for result in results {
            println!();

            let size_mib = float_fmt(result.bench_size() as f64 / 1048576., 2);
            let sample_count = result.sample_count();
            let percentile = result.percentile(90);
            let stddev = float_fmt(result.stddev(), 2);
            let median = float_fmt(result.median(), 2);

            println!("  {} ({} runs @ {} MiB):", result.name(), sample_count, size_mib);
            println!("    {}ms median (90% < {}ms) +-{}ms", median, percentile, stddev);
        }
    }
}

// Format floating point numbers.
fn float_fmt(float: f64, decimals: i32) -> f64 {
    let pow = f64::powi(10., decimals);
    (float * pow).round() / pow
}
