# vtebench

A tool for benchmarking terminal emulator PTY read performance.

## Disclaimer

This benchmark is not sufficient to get a general understanding of the
performance of a terminal emulator. It lacks support for critical factors like
framerate or latency. The only factor this benchmark stresses is the speed at
which a terminal reads from the PTY. If you do not understand what this means,
please do not jump to any conclusions from the results of this benchmark.

## Usage

vtebench accepts benchmarks as executables and uses their stdout as benchmark
payload. By default benchmarks are read from the `./benchmarks` directory, which
contains a good selection of benchmarks already. Benchmarks in vtebench are
defined as a directory with a `benchmark` and an optional `setup` executable.

To just run all the default benchmarks in the repository, you can run the
following after setting up a Rust toolchain:

```
cargo run --release
```

## Plotting

vtebench contains a script for automatically plotting results using `gnuplot`.
To do this you first need to output the benchmark results in the `.dat` format:

```
cargo run --release -- --dat results.dat
```

After having generated the `.dat` file, you can then pass it to the `gnuplot.sh`
script to generate the SVG plot:

```
./gnuplot.sh results.dat output.svg
```

You can combine any number of results by passing them to the gnuplot script:

```
./gnuplot.sh *.dat output.svg
```

## Contributing Benchmarks

If you have found benchmarks that might provide insightful information, or show
significant differences between different terminals and version, you can send a
pull request to add them to the default benchmark collection.

To do so, you just need to create a new directory in the `./Benchmarks`
directory and add an executable for your `setup` and `benchmark` functions.
The stdout of the benchmark will automatically be repeated to fill a reasonable
minimum sample size, so make sure to take that into account and move everything
into `setup` that should only be done once.
