//! Gnuplot .dat file format.

use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::bench::Results;
use crate::format::Format;

/// DAT file format.
pub struct DatFormat {
    path: PathBuf,
}

impl DatFormat {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn write(&self, results: &[Results]) -> io::Result<()> {
        let text = results_to_cols(results);
        let mut file = File::create(&self.path)?;
        file.write_all(text.as_bytes())
    }
}

impl Format for DatFormat {
    fn format(&self, results: &[Results]) {
        if let Err(error) = self.write(results) {
            eprintln!("Unable to write DAT file: {}", error);
        }
    }
}

/// Convert results to a string with multiple columns.
fn results_to_cols(results: &[Results]) -> String {
    let mut output = String::new();

    // Write column headers.
    for result in results {
        output = output + result.name() + " ";
    }
    output.push('\n');

    // Write benchmark results.
    let max_samples = results.iter().map(|result| result.sample_count()).max().unwrap_or(0);
    for i in 0..max_samples {
        for result in results {
            // Convert sample to string, using `_` as placeholder value.
            let sample = result
                .samples()
                .get(i)
                .map(|sample| sample.to_string())
                .unwrap_or_else(|| String::from("_"));
            output += &(sample + " ");
        }
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_results() {
        let results1 = Results::new("one".into(), 3, vec![1, 2, 3]);
        let results2 = Results::new("two".into(), 3, vec![3, 2]);

        let formatted = results_to_cols(&[results1, results2]);

        assert_eq!(formatted, "one two \n1 3 \n2 2 \n3 _ \n");
    }
}
