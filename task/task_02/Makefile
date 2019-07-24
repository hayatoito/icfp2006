# Task02: Login to system, using main.um, which is extracted by task01

# > um main.um

# See /Users/hayato/drive/archive/2014/scala/main/icfpc2006/resources:

all: run

run:
	cargo run --release --bin my-icfp2006-um -- ../task_01/main.um

session-log:
	echo "See log.txt"
	cat log.txt

run-with-input:
	: # (cat ./input_00.txt; cat /dev/stdin) | cargo run --release --bin um -- ../task01/main.um
	cat ./input_00.txt | cargo run --release --bin my-icfp2006-um -- ../task_01/main.um > output_00.txt

# Sample:
# CCLXXX   words(XXXII) = "penguin"
# CCLXXXV  words(XXXIII) = "polynomial"
extract-passowrd:
	./extract_password.sh

blute-force:
	bash ./blute_force.sh | tee output_blute_force.txt

# => password: xyzzy

.PHONY: all clean
