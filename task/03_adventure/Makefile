# Task03:

run:
	cargo run --release -- um ../main.um

run-with-input:
	(cat ./input_00.txt; cat /dev/stdin) | cargo run --release -- um --print-stdin ../main.um

run-with-input-1:
	(cat ./input_01.txt; cat /dev/stdin) | cargo run --release -- um --print-stdin ../main.um


# Adventure Part 1

run-adventure-1:
	(cat ./input_00.txt; echo "./adventure"; cat /dev/stdin) | tee adventure_input_log_1.txt | cargo run --release -- um ../main.um

solve-adventure-1:
	cargo run --release -- -v adventure solve1 ../../task/main.um | tee solve-adventure.log.txt

# Adventure Part 2

run-with-input-2:
	(cat ./input_02.txt; cat /dev/stdin) | cargo run --release -- um --print-stdin ../main.um

solve-adventure-2:
	cargo run --release -- -v adventure solve2 ../../task/main.um



## Part 1


# Part2


# >: ls
# ls
# 54th Street and Ridgewood Court

# You are standing at the corner of 54th Street and Ridgewood
# Court. From here, you can go east.
# There is a /etc/passwd here.
# Underneath the /etc/passwd, there is a self-addressed note.
# Underneath the note, there is a (broken) downloader.
# Underneath the downloader, there is a (broken) uploader.

# >: cat /etc/passwd
# cat /etc/passwd
# The /etc/passwd is some kind of lost inode. It reads:
# howie:xyzzy:Howard Curry:/home/howie
# yang:U+262F:Y Yang:/home/yang
# hmonk:COMEFROM:Harmonious Monk:/home/hmonk.
# Also, it is in pristine condition.


.PHONY: all clean
