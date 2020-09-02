//! Benchmark output formats.

pub mod stdout;

use crate::bench::Results;

/// Trait for displaying the benchmark results.
pub trait Format {
    /// Output the benchmark results.
    fn format(&self, results: &[Results]);
}
