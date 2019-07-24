#!/bin/bash
cat password.txt | \
  while read p ; do
    echo "try password: $p"
    output=$(printf "howie\n$p\n" | cargo run --release --bin my-icfp2006-um -- --print-stdin ../task_01/main.um)
    echo "output: $output"
    echo
  done
