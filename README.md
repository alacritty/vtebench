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
understanding, **not** performance. To benchmark the currently running terminal
then, something like this would work:

```sh
vtebench -w $(tput cols) -h $(tput lines) alt-screen-random-write \
    > /tmp/100mb.vte

time cat /tmp/100mb.vte
```

In the future, it would be nice to have a script to automate generating all of
the tests, running them several times and generate statistics, and print all the
results in a machine+human friendly format.

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
