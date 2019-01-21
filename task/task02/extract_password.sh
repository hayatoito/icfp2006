#!/bin/zsh
cat output_00.txt | egrep -E -o 'words\(.*\) = ".*"' | while read _ _ a ; do echo ${a:gs/\"/}; done > password.txt
