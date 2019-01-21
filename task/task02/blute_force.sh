#!/bin/zsh
cat password.txt | \
  while read p ; do
    echo "try password: $p"
    output=$(echo "howie\n$p" | cargo run --release --bin um -- --print-stdin ../task01/main.um)
    echo "output: $output"
    echo
  done
