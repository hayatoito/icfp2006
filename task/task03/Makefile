# Task03:

run:
	cargo run --release --bin um -- ../task01/main.um

run-with-input:
	(cat ./input_00.txt; cat /dev/stdin) | cargo run --release --bin um -- ../task01/main.um

# Adventure

run-adventure:
	(cat ./input_00.txt; echo "./adventure"; cat /dev/stdin) | tee adventure_input_log_1.txt | cargo run --release --bin um -- ../task01/main.um

run-adventure-2:
	(cat ./adventure_input_log_1.txt; cat /dev/stdin) | tee adventure_input_log_2.txt | cargo run --release --bin um -- ../task01/main.um


# combine
# (broken) radio + anntena => (broken) radio
# (broken) radio + blue transistor => radio
# (broken) processor + cache => processor
# (broken) motherboard + screw => (broken) motherboard
# (broken) A-1920-IXB + processor => (broken) A-1920-IXB

.PHONY: all clean
