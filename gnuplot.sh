#!/usr/bin/env bash

# Plot every sample of all benchmarks.

# Make sure gnuplot is installed.
if ! [ -x "$(command -v gnuplot)" ]; then
    echo "command not found: gnuplot"
    exit 1
fi

# Ensure at least one input and output file is present.
if [ $# -lt 2 ]; then
    echo "Usage: gnuplot.sh <INPUT_FILES>... <OUTPUT_FILE>"
    exit 1
fi

# Use last argument as output file.
output_index=$#
output_file=${!output_index}

# Setup gnuplot script with output format and file.
gnuplot_script="\
set terminal svg noenhanced size 1000,750
set output \"${output_file}\"
set xlabel \"samples\"
set ylabel \"milliseconds (lower is better)\"
plot "

# Add all columns for the input file to the gnuplot script.
for input_index in $(seq 1 $(($# - 1))); do
    input_file=${!input_index}
    num_cols=$(cat "$input_file" | head -n 1 | awk '{ print NF }')
    gnuplot_script+="for[col = 1:${num_cols}] \
        \"$input_file\" \
        using col \
        with linespoint \
        title \"$input_file: \".columnhead(col),"
done
gnuplot_script=${gnuplot_script::-1}

# Plot everything.
echo -e "$gnuplot_script" | gnuplot
