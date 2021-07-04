# Task02: Login to system, using main.um, which is extracted by task01

# > um main.um

all: run

run:
	cargo run --release -- um ../main.um

session-log:
	echo "See log.txt"
	cat log.txt

run-with-input:
	: # (cat ./input_00.txt; cat /dev/stdin) | cargo run --release --bin um -- ../main.um
	cat ./input_00.txt | cargo run --release -- um ../main.um > output_00.txt

# [2020-07-13 Mon] solve by qbasic
solve:
	cargo run --release -- qbasic ../main.um

# found match!! for user ftd
# password: falderal90

# found match!! for user ohmega
# password: bidirectional

# found match!! for user howie
# password: xyzzy


# [Depreacted] Old way
# Sample:
# CCLXXX   words(XXXII) = "penguin"
# CCLXXXV  words(XXXIII) = "polynomial"
# extract-passowrd:
# 	./extract_password.sh

# blute-force-without-qbasic:
# 	bash ./blute_force.sh | tee output_blute_force.txt

# => password: xyzzy


.PHONY: all clean
