vtebench
========

A tool for generating terminal benchmarks

## Usage

The general usage pattern is

```
vtebench -w $(tput cols) -h $(tput lines) [-c|-b=BYTES|-t=TERM] <benchmark>
```

Terminal protocol will be output to `stdout`. Output **must be** directed into a
file rather than used directly to benchmark. `vtebench` is written for ease of
understanding, **not** performance.

To generate the most basic commands, the
[`generate-benchmarks.sh`](./generate-benchmarks.sh) script can be used. This
should be run in the project's root directory and will output the benchmark
files to `target/benchmarks`.

After the files have been generated, the performance can be measured with
[perf](https://perf.wiki.kernel.org/index.php/Main_Page), or
[hyperfine](https://github.com/sharkdp/hyperfine) on macOS or Windows:

```sh
perf stat -r 10 cat target/benchmarks/alt-screen-random-write.vte
hyperfine --show-output "cat target/benchmarks/scrolling.vte"
```

Great instructions on how to reliably generate consistent benchmarks can be
found in the [llvm documentation](https://llvm.org/docs/Benchmarking.html).
Usually it is not required to limit execution to specific cores, but the other
instructions will greatly help with consistency.

### The `-b|--bytes` flag

It's important to generate sufficient output to test the terminal. If the test
only takes 1ms to complete, you lack statistical significance. As a guideline,
`time cat <script>` should take at least 1 second. How much data is needed to
get there will vary greatly by terminal.

## Contributing

If you wish to add a new test, do the following:

1. Add a new function in _bench.rs_ with the same pattern as an existing
   function.
2. Add a subcommand to run it in the `Benchmark` enum within _cli.rs_.
3. Handle the subcommand in _main.rs_.

If there are escape codes that are not yet supported on `Context` it is quite
helpful to reference the `terminfo` man page and cross reference with the
`terminfo` **crate**'s `capability` submodule documentation. Each capability
name has a corresponding type in that submodule.
