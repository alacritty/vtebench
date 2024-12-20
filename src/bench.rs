//! Benchmark execution and analysis.

use std::cmp::min;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::ffi::OsStr;
use std::fmt::{self, Display, Formatter};
use std::io::{self, StdoutLock, Write};
use std::iter;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

use walkdir::WalkDir;

/// Errors during the benchmark process.
#[derive(Debug)]
pub enum Error {
    /// No bytes are present in the benchmark.
    Empty,
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Empty => write!(f, "empty benchmark"),
        }
    }
}

/// Find all benchmarks recursively in the specified location.
pub fn find_benchmarks(paths: &[PathBuf]) -> Vec<BenchmarkLoader> {
    let mut benchmarks: HashMap<String, (Option<PathBuf>, Option<PathBuf>)> = HashMap::new();

    for path in paths {
        for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.file_name() == "setup" || entry.file_name() == "benchmark")
        {
            let file_name = entry.file_name().to_string_lossy().into_owned();
            let path = entry.into_path();

            if let Some(name) = path.parent().and_then(|path| path.file_name()) {
                let name = name.to_string_lossy().to_string();
                let bench = benchmarks.entry(name).or_default();
                match file_name.as_str() {
                    "setup" => bench.0 = Some(path),
                    "benchmark" => bench.1 = Some(path),
                    _ => unreachable!(),
                }
            }
        }
    }

    // Combine name/setup/benchmark into a single benchmark loader.
    let mut benchmarks: Vec<BenchmarkLoader> = benchmarks
        .drain()
        .filter_map(|(name, (setup_path, bench_path))| {
            bench_path.map(|bench_path| BenchmarkLoader { name, setup_path, bench_path })
        })
        .collect();

    // Sort benchmarks to ensure consistent order.
    benchmarks.sort_unstable_by(|a, b| a.name.cmp(&b.name));

    benchmarks
}

/// Benchmark loader.
///
/// This loader stores all state necessary to initialize the benchmark, without allocating the
/// entire benchmark ahead of execution.
pub struct BenchmarkLoader {
    setup_path: Option<PathBuf>,
    bench_path: PathBuf,
    name: String,
}

impl BenchmarkLoader {
    /// Create the benchmark.
    ///
    /// This will allocate the necessary buffer to run the benchmark directly from memory.
    pub fn load(self, min_bytes: usize) -> Result<Benchmark, Error> {
        println!("Loading {}...", self.name);
        Benchmark::new(self.name, self.setup_path.as_ref(), self.bench_path, min_bytes)
    }
}

/// A single benchmark with all data cached.
pub struct Benchmark {
    /// Benchmark initialization data.
    setup: Vec<u8>,

    /// Benchmark execution data.
    benchmark: Vec<u8>,

    /// Name of the benchmark.
    name: String,
}

impl Benchmark {
    /// Load a benchmark.
    ///
    /// If the benchmark is too small, it will be repeated in full until the `min_bytes` parameter
    /// is reached or exceeded.
    pub fn new<SP, BP>(
        name: String,
        setup_path: Option<SP>,
        bench_path: BP,
        min_bytes: usize,
    ) -> Result<Self, Error>
    where
        SP: AsRef<OsStr>,
        BP: AsRef<OsStr>,
    {
        // Execute setup shell script and capture its output.
        let setup = match setup_path {
            Some(path) => Command::new(path).output().map(|out| out.stdout).unwrap_or_default(),
            None => Vec::new(),
        };

        // Execute benchmark shell script and capture its output.
        let bench = Command::new(bench_path).output().map(|out| out.stdout).unwrap_or_default();

        // If there's no data, the minimum benchmark size cannot be reached.
        if min_bytes == 0 || bench.is_empty() {
            return Err(Error::Empty);
        }

        // Repeat until `min_bytes` is reached.
        let count = (min_bytes - 1) / bench.len() + 1;
        let bytes = iter::repeat(bench).take(count).flatten().collect();

        Ok(Self { benchmark: bytes, setup, name })
    }

    /// Execute the benchmark.
    ///
    /// This will write the entire benchmark to STDOUT multiple times.
    pub fn run(&self, warmup_runs: usize, max_secs: u64, max_samples: Option<usize>) -> Results {
        // Lock stdout to ensure consistency.
        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        // Write benchmark as warmup to fill PTY buffer.
        for _ in 0..warmup_runs {
            self.run_sample(&mut stdout);
        }

        let mut samples = Vec::new();

        let max_samples = max_samples.unwrap_or(usize::MAX);
        let end = Instant::now() + Duration::from_secs(max_secs);
        for _ in (0..max_samples).take_while(|_| Instant::now() < end) {
            let duration = self.run_sample(&mut stdout);
            samples.push(duration.as_millis() as usize);
        }

        // Reset.
        let _ = stdout.write_all(b"\x1bc");
        let _ = stdout.flush();

        Results::new(self.name.clone(), self.benchmark.len(), samples)
    }

    /// Run a single benchmark sample.
    fn run_sample(&self, stdout: &mut StdoutLock) -> Duration {
        // Reset everything before starting.
        let _ = stdout.write_all(b"\x1bc");
        let _ = stdout.flush();

        // Setup benchmark.
        let _ = stdout.write_all(&self.setup);

        // Execute the benchmark.
        let start = Instant::now();
        let _ = stdout.write_all(&self.benchmark);
        let _ = stdout.flush();

        // Measure how long the writing blocked for.
        Instant::now() - start
    }
}

/// Benchmark results.
pub struct Results {
    /// Sorted samples for calculations like median.
    sorted_samples: Vec<usize>,

    /// Samples ordered chronologically.
    samples: Vec<usize>,

    /// Number of bytes in one sample of the benchmark.
    bench_size: usize,

    /// Name of the benchmark.
    name: String,
}

#[allow(unused)]
impl Results {
    pub fn new(name: String, bench_size: usize, mut samples: Vec<usize>) -> Self {
        // Assure the vector is never empty to simplify the math.
        if samples.is_empty() {
            samples.push(0);
        }

        let mut sorted_samples = samples.clone();
        sorted_samples.sort_unstable();

        Results { sorted_samples, samples, bench_size, name }
    }

    /// Name of the benchmark.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// All recorded samples.
    pub fn samples(&self) -> &[usize] {
        self.samples.as_slice()
    }

    /// Size of the benchmark in bytes.
    pub fn bench_size(&self) -> usize {
        self.bench_size
    }

    /// Number of samples taken.
    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }

    /// Fastest benchmark sample in milliseconds.
    pub fn min(&self) -> usize {
        self.samples.iter().min().copied().unwrap_or_default()
    }

    /// Slowest benchmark sample in milliseconds.
    pub fn max(&self) -> usize {
        self.samples.iter().max().copied().unwrap_or_default()
    }

    /// Mean execution time per sample in milliseconds.
    pub fn mean(&self) -> f64 {
        self.samples.iter().sum::<usize>() as f64 / self.samples.len() as f64
    }

    /// Median execution time per sample in milliseconds.
    pub fn median(&self) -> f64 {
        let len = self.samples.len();
        (self.sorted_samples[(len - 1) / 2] as f64 + self.sorted_samples[len / 2] as f64) / 2.
    }

    /// Variance of the execution time in milliseconds.
    pub fn variance(&self) -> f64 {
        if self.samples.len() < 2 {
            return 0.;
        }

        let mean = self.mean();
        let len = self.samples.len();
        self.samples.iter().map(|&s| f64::powi(s as f64 - mean, 2)).sum::<f64>() / (len - 1) as f64
    }

    /// Standard deviation of the execution time in milliseconds.
    pub fn stddev(&self) -> f64 {
        self.variance().sqrt()
    }

    /// Execution time in millisecond that the specified percentage of samples lie below.
    pub fn percentile(&self, mut percentile: usize) -> usize {
        percentile = min(percentile, 100);

        let index = ((self.samples.len() * percentile + 99) / 100).saturating_sub(1);
        self.sorted_samples[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64;

    #[test]
    fn max() {
        let results = new_results(vec![6, 3, 1, 9, 8]);
        assert_eq!(results.max(), 9);
    }

    #[test]
    fn min() {
        let results = new_results(vec![6, 3, 1, 9, 8]);
        assert_eq!(results.min(), 1);
    }

    #[test]
    fn mean() {
        let results = new_results(vec![20, 30, 40, 80, 100, 100]);
        float_eq(results.mean(), 61.666666666666664);
    }

    #[test]
    fn median() {
        let results = new_results(vec![20, 30, 55, 60, 100, 100]);
        float_eq(results.median(), 57.5);

        let results = new_results(vec![20, 30, 40, 60, 80, 100, 100]);
        float_eq(results.median(), 60.);
    }

    #[test]
    fn variance() {
        let results = new_results(vec![4, 8, 8, 8, 10, 10]);
        float_eq(results.variance(), 4.8);

        let results = new_results(vec![3]);
        float_eq(results.variance(), 0.);
    }

    #[test]
    fn stddev() {
        let results = new_results(vec![4, 8, 8, 8, 10, 10]);
        float_eq(results.stddev(), 2.1908902300206643);

        let results = new_results(vec![3]);
        float_eq(results.stddev(), 0.);
    }

    #[test]
    fn percentile() {
        let results = new_results(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(results.percentile(110), 10);
        assert_eq!(results.percentile(100), 10);
        assert_eq!(results.percentile(90), 9);
        assert_eq!(results.percentile(80), 8);
        assert_eq!(results.percentile(70), 7);
        assert_eq!(results.percentile(60), 6);
        assert_eq!(results.percentile(50), 5);
        assert_eq!(results.percentile(40), 4);
        assert_eq!(results.percentile(30), 3);
        assert_eq!(results.percentile(20), 2);
        assert_eq!(results.percentile(10), 1);
        assert_eq!(results.percentile(0), 1);
    }

    fn new_results(samples: Vec<usize>) -> Results {
        Results { sorted_samples: samples.clone(), samples, bench_size: 0, name: String::new() }
    }

    fn float_eq(f1: f64, f2: f64) {
        if (f1 - f2).abs() >= f64::EPSILON {
            panic!("float assertion failed: {} != {}", f1, f2);
        }
    }
}
