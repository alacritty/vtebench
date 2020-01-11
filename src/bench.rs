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

use cli::{Benchmark, Options};
use context::Context;
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
        let (l, c) = (rng.gen_range(0, h), rng.gen_range(0, w - 1));
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

fn build_scroll_buf(fill: bool, cols: u16) -> Vec<u8> {
    if fill {
        let mut buf = Vec::new();
        for _ in 0..(cols - 1) {
            buf.push(b'a');
        }
        buf.push(b'\n');
        buf
    } else {
        YES.to_owned()
    }
}

pub fn scrolling_in_region<W: Write>(ctx: &mut Context<W>, options: &Options) -> Result<usize> {
    let mut written = 0;
    let h = options.height;

    let (fill_lines, lines_from_top, lines_from_bottom) = match options.benchmark {
        Benchmark::ScrollingInRegion { fill_lines, lines_from_top, lines_from_bottom } => {
            (fill_lines, lines_from_top, lines_from_bottom)
        },
        _ => panic!("Wrong benchmark"),
    };

    // First, setup the scroll region. Use as many lines as there are available, less 1.
    written += ctx.csr(lines_from_top, h - 2 - lines_from_bottom)?;
    for i in 0..lines_from_bottom {
        written += ctx.cup(h - 1 - i, 0)?;
        let message = format!("REGION BOTTOM {}", i);
        ctx.write_all(&message.into_bytes())?;
    }
    written += ctx.cup(lines_from_top, 0)?;

    let buf = build_scroll_buf(fill_lines, options.width);
    while written < options.bytes {
        ctx.write_all(&buf)?;
        written += buf.len();
    }

    ctx.csr(0, h)?;
    ctx.sgr0()?;

    Ok(written)
}

pub fn scrolling<W: Write>(ctx: &mut Context<W>, options: &Options) -> Result<usize> {
    let mut written = 0;

    let fill_lines = match options.benchmark {
        Benchmark::Scrolling { fill_lines } => fill_lines,
        _ => panic!("Wrong benchmark"),
    };

    let buf = build_scroll_buf(fill_lines, options.width);
    while written < options.bytes {
        ctx.write_all(&buf)?;
        written += buf.len();
    }
    ctx.sgr0()?;

    Ok(written)
}

pub fn unicode_random_write<W: Write>(ctx: &mut Context<W>, options: &Options) -> Result<usize> {
    let mut written = 0;
    let mut rng = rand::thread_rng();
    let h = options.height;
    let w = options.width;

    while written < options.bytes {
        let (l, c) = (rng.gen_range(0, h), rng.gen_range(0, w - 1));

        written += ctx.cup(l, c)?;
        if options.colorize {
            written += ctx.setaf(rng.gen_range(0, 8))?;
        }

        let unicode_value = rng.gen_range(0, u16::max_value());
        let unicode = String::from_utf16_lossy(&[unicode_value]).into_bytes();
        ctx.write_all(&unicode)?;
        written += unicode.len();
    }

    Ok(written)
}
