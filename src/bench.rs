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
use std::io::Write;

use rand::{self, Rng};

use context::Context;
use cli::Options;
use result::Result;

static YES: &[u8] = b"\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\
    \ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\
    \ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\
    \ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\
    \ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\
    \ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\
    \ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\
    \ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\
    \ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\
    \ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\
    \ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\
    \ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\
    \ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny\ny";

pub fn alt_screen_random_write<W: Write>(ctx: &mut Context<W>, options: &Options) -> Result<usize> {
    let mut written = 0;
    let mut rng = rand::thread_rng();
    let h = options.height;
    let w = options.width;

    ctx.smcup()?;

    while written < options.bytes {
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

pub fn scrolling_in_region<W: Write>(ctx: &mut Context<W>, options: &Options) -> Result<usize> {
    let mut written = 0;
    let h = options.height;


    // First, setup the scroll region. Use as many lines as there are available, less 1.
    written += ctx.csr(0, h - 2)?;
    written += ctx.cup(h - 1, 0)?;
    ctx.write_all(b"REGION BOTTOM")?;
    while written < options.bytes {
        ctx.write_all(YES)?;
        written += YES.len();
    }

    ctx.csr(0, h)?;
    ctx.sgr0()?;

    Ok(written)
}
