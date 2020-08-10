use structopt::StructOpt;

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

    #[structopt(
        long = "term",
        help = "terminal name (terminfo)",
        default_value = "xterm-256color"
    )]
    pub term: String,

    #[structopt(subcommand)]
    pub benchmark: Benchmark,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "vtebench", about = "Benchmark Terminal Emulators")]
pub enum Benchmark {
    #[structopt(
        name = "alt-screen-random-write",
        about = "Set alt screen; repeatedly: pick random location, write ascii"
    )]
    AltScreenRandomWrite,

    #[structopt(
        name = "unicode-random-write",
        about = "Repeatedly picks location and writes unicode character"
    )]
    UnicodeRandomWrite,

    #[structopt(
        name = "scrolling-in-region",
        about = "Repeatedly outputs 'y\\n' within a scrolling region"
    )]
    ScrollingInRegion {
        #[structopt(long = "fill-lines", help = "fills lines instead of using y\\n")]
        fill_lines: bool,

        #[structopt(
            long = "lines-from-top",
            help = "how far scrolling region extends from top",
            default_value = "0"
        )]
        lines_from_top: u16,

        #[structopt(
            long = "lines-from-bottom",
            help = "how far scrolling region extends from bottom",
            default_value = "0"
        )]
        lines_from_bottom: u16,
    },

    #[structopt(name = "scrolling", about = "Repeatedly outputs 'y\\n'")]
    Scrolling {
        #[structopt(long = "fill-lines", help = "fills lines instead of using y\\n")]
        fill_lines: bool,
    },
}
