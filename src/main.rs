//! A tool for generating benchmark scripts for terminal emulators
//!
//! This program is intended to be run and its output piped into a file, and the
//! file can simply be `cat`ed from the terminal emulator under test. This
//! ensures that the terminal is being benchmarked and not this vtebench
//! application.
use std::io::{self, BufWriter};

use structopt::StructOpt;
use terminfo::Database;

mod bench;
mod cli;
mod context;
mod result;

use cli::{Benchmark, Options};
use context::Context;
use result::Result;

fn main() {
    run().unwrap();
}

fn run() -> Result<()> {
    // Load command line options
    let options = Options::from_args();

    // Load terminfo database
    let db = Database::from_name(&options.term)?;

    // Get I/O handle
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut out = BufWriter::new(&mut out);

    // Create the output context
    let mut ctx = Context { out: &mut out, db: &db, buf: Vec::new(), rng: rand::thread_rng() };

    // Run the requested benchmark
    match options.benchmark {
        Benchmark::AltScreenRandomWrite => bench::alt_screen_random_write(&mut ctx, &options)?,
        Benchmark::UnicodeRandomWrite => bench::unicode_random_write(&mut ctx, &options)?,
        Benchmark::ScrollingInRegion { .. } => bench::scrolling_in_region(&mut ctx, &options)?,
        Benchmark::Scrolling { .. } => bench::scrolling(&mut ctx, &options)?,
    };

    // Success!
    Ok(())
}
