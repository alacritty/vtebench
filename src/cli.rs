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

/// Command line options
#[derive(StructOpt, Debug)]
#[structopt(name = "vtebench", about = "Benchmark Terminal Emulators")]
pub struct Options {
    #[structopt(short = "w", help = "width of terminal", default_value = "80")]
    pub width: u16,

    #[structopt(short = "h", help = "height of terminal", default_value = "24")]
    pub height: u16,

    #[structopt(
        short = "b",
        long = "bytes",
        help = "minimum bytes to output; actual value may be slightly higher",
        default_value = "1048576"
    )]
    pub bytes: usize,

    #[structopt(short = "c", help = "colorized output (not all tests support)")]
    pub colorize: bool,

    #[structopt(long = "term", help = "terminal name (terminfo)", default_value = "xterm-256color")]
    pub term: String,

    #[structopt(subcommand)]
    pub benchmark: Benchmark,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "vtebench", about = "Benchmark Terminal Emulators")]
pub enum Benchmark {
    #[structopt(
        name = "alt-screen-random-write",
        help = "Set alt screen; repeatedly: pick random location, write ascii"
    )]
    AltScreenRandomWrite,

    #[structopt(
        name = "scrolling-in-region",
        help = "Line feed within a scrolling region"
    )]
    ScrollingInRegion,

    #[structopt(
        name = "scrolling",
        help = "Repeatedly outputs 'y\\n'"
    )]
    Scrolling,
}
