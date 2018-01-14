// Copyright 2016 Joe Wilm
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A tool for generating benchmark scripts for terminal emulators
//!
//! This program is intended to be run and its output piped into a file, and the
//! file can simply be `cat`ed from the terminal emulator under test. This
//! ensures that the terminal is being benchmarked and not this vtebench
//! application.
extern crate rand;
extern crate structopt;

#[macro_use] extern crate failure;
#[macro_use] extern crate structopt_derive;
#[macro_use] extern crate terminfo;

use std::io::{self, BufWriter};

use structopt::StructOpt;
use terminfo::Database;

mod bench;
mod cli;
mod context;
mod result;

use cli::{Options, Benchmark};
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
