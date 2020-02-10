#!/bin/bash

# All benchmarks are generated to ./target/benchmarks
target_dir="target/benchmarks"
mkdir -p "$target_dir"

# Generate output file for one benchmark
function generate_benchmark {
    timeout=$1
    benchname=$2

    # All remaining parameters are benchmark parameters
    params=""
    if [ $# -gt 2]; then
      for i in $(seq 3 $#); do
          params+="${!i} "
      done
    fi

    # Target file path
    file="$target_dir/$benchname.vte"

    # Start benchmark $benchname and kill it after $timeout seconds
    target/release/vtebench -h $(tput lines) -w $(tput cols) -c --bytes 999999999999 "$benchname" $params | tee "$file" &
    sleep "$timeout"
    pkill vtebench

    # Wait for application to finish before writing to file
    sleep 1

    # Since we kill the benchmark, we're doing the resetting ourself
    printf "\ec" >> "$file"
}

# Compile vtebench before starting anything time sensitive
cargo build --release

# ASRW benchmark has big differences running from file, so we use a higher timeout here
generate_benchmark 15 alt-screen-random-write
printf "\ec"

generate_benchmark 5 scrolling
printf "\ec"

generate_benchmark 5 scrolling-in-region --lines-from-bottom 1
printf "\ec"
