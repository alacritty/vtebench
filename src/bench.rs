use std::io::Write;

use rand::{self, Rng};

use context::Context;
use cli::Options;
use result::Result;

pub fn alt_screen_random_write<W: Write>(ctx: &mut Context<W>, options: &Options) -> Result<usize> {
    let mut written = 0;
    let mut rng = rand::thread_rng();
    let h = options.height;
    let w = options.width;
    let mut buf = Vec::<u8>::with_capacity(w as usize);

    ctx.smcup()?;

    while written < options.bytes {
        buf.clear();
        let (l, c) = (rng.gen_range(0, h), rng.gen_range(0, w - 2));
        let space = w - c;
        let to_write = rng.gen_range(0, space);

        written += ctx.cup(l, c)?;
        if options.colorize {
            written += ctx.setaf(rng.gen_range(0, 8))?;
        }
        written += ctx.write_ascii(to_write as _)?;
    }

    ctx.sgr0()?;
    ctx.rmcup()?;

    Ok(written)
}
