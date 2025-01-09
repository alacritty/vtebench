#!/usr/bin/env bash

# Plot a summary for each benchmark.

box_width=2
gap_size=2

# Make sure gnuplot is installed.
if ! [ -x "$(command -v gnuplot)" ]; then
    printf "command not found: gnuplot\n"
    exit 1
fi

# Ensure at least one input and output file is present.
if [ $# -lt 2 ]; then
    printf "Usage: gnuplot.sh <INPUT_FILES>... <OUTPUT_FILE>\n"
    exit 1
fi

# Use last argument as output file.
output_index=$#
output_file=${!output_index}

# Setup gnuplot script with output format and file.
gnuplot_script="\
set terminal svg noenhanced size 1000,750 background rgb 'white'
set output \"${output_file}\"
set ylabel \"milliseconds (lower is better)\"
set style fill solid 0.25 border -2
set style boxplot nooutliers
set style data boxplot
set key above
set boxwidth 1.8\n"

num_inputs=$(($# - 1))
bench_width=$((box_width * num_inputs + gap_size))

# Use column headers as x tic labels.
gnuplot_tics="set xtics scale 0 ("
index=1
for column_label in $(cat "$1" | head -n 1); do
    value=$((index * bench_width + num_inputs / 2))
    gnuplot_tics+="\"$column_label\" $value,"

    # Draw separator grid between benchmarks.
    if [ $index -ne 1 ]; then
        separator_value=$((index * bench_width - box_width / 2 - gap_size / 2))
        gnuplot_script+="set arrow $index \
            from $separator_value, graph 0 \
            to $separator_value, graph 1 \
            nohead dt \".\" linecolor \"#888888\"\n"
    fi

    index=$((index + 1))
done
gnuplot_tics="${gnuplot_tics::${#gnuplot_tics}-1}) rotate by 315 left\n"
gnuplot_script+=$gnuplot_tics

# Get the mean for all columns in every file.
gnuplot_script+="plot "
for input_index in $(seq 1 $num_inputs); do
    input_file=${!input_index}
    input_filename=$(basename "$input_file")
    num_cols=$(cat "$input_file" | head -n 1 | wc -w)
    gnuplot_script+="for[col = 1:${num_cols}] \
        \"$input_file\" \
        using (col * $bench_width + $((box_width * (input_index - 1)))):col \
        title (col == 1 ? \"$input_filename\" : \"\") \
        linecolor $input_index,"
done
gnuplot_script=${gnuplot_script::${#gnuplot_script}-1}

# Plot everything.
printf "$gnuplot_script" | gnuplot
