//! Benchmark output formats.

mod dat;
mod stdout;

pub use crate::format::dat::DatFormat;
pub use crate::format::stdout::StdoutFormat;

use crate::bench::Results;

/// Trait for displaying the benchmark results.
pub trait Format {
    /// Output the benchmark results.
    fn format(&self, results: &[Results]);
}
