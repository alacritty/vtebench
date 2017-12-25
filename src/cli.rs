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

    #[structopt(long = "term", help = "height of terminal", default_value = "xterm-256color")]
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
}
