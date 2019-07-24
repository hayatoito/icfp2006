# Task03:

run:
	cargo run --release --bin my-icfp2006-um -- ../task_01/main.um

run-with-input:
	(cat ./input_00.txt; cat /dev/stdin) | cargo run --release --bin my-icfp2006-um -- --print-stdin ../task_01/main.um

run-with-input-1:
	(cat ./input_01.txt; cat /dev/stdin) | cargo run --release --bin my-icfp2006-um -- --print-stdin ../task_01/main.um


# Adventure Part 1

run-adventure:
	(cat ./input_00.txt; echo "./adventure"; cat /dev/stdin) | tee adventure_input_log_1.txt | cargo run --release --bin my-icfp2006-um -- ../task_01/main.um

solve-adventure:
	cargo run --release --bin my-icfp2006-task -- -v task03 solve-adventure ../../task/task_01/main.um | tee solve-adventure.log.txt

# Adventure Part 2

run-with-input-2:
	(cat ./input_02.txt; cat /dev/stdin) | cargo run --release --bin my-icfp2006-um -- --print-stdin ../task_01/main.um

solve-adventure-2:
	cargo run --release --bin my-icfp2006-task -- -v task03 solve-adventure-2 ../../task/task_01/main.um



## Part 1

#           [pile of junk]
# [salon E] [start]

# [start]:
#   pamphret
#   menifesto
# [salon E]:
#   bullet-point
#   (broken) slides.ppt
# [pile of junk]:
#   Underneath the bolt, there is a spring.
#   Underneath the spring, there is a button.
#   Underneath the button, there is a (broken) processor.
#   Underneath the processor, there is a red pill.
#   Underneath the pill, there is a (broken) radio.
#   Underneath the radio, there is a cache.
#   Underneath the cache, there is a blue transistor.
#   Underneath the transistor, there is an antenna.
#   Underneath the antenna, there is a screw.
#   Underneath the screw, there is a (broken) motherboard.
#   Underneath the motherboard, there is a (broken) A-1920-IXB.
#   Underneath the A-1920-IXB, there is a red transistor.
#   Underneath the transistor, there is a (broken) keypad.
#   Underneath the keypad, there is some trash.


# can't take button.  you can't carry any more items.

# You are carrying:
# a spring,
# a bolt,
# a (broken) slides.ppt,
# a bullet-point,
# a manifesto and
# a pamphlet.




#   Underneath the button, there is a (broken) processor.
#   Underneath the processor, there is a red pill.
#   Underneath the pill, there is a (broken) radio.
#   Underneath the radio, there is a cache.
#   Underneath the cache, there is a blue transistor.
#   Underneath the transistor, there is an antenna.
#   Underneath the antenna, there is a screw.
#   Underneath the screw, there is a (broken) motherboard.
#   Underneath the motherboard, there is a (broken) A-1920-IXB.
#   Underneath the A-1920-IXB, there is a red transistor.
#   Underneath the transistor, there is a (broken) keypad.
#   Underneath the keypad, there is some trash.


# combine

# [2019-07-06 Sat]
# (broken) slides.ppt + bullet-point => slides.ppt

# take button





# (broken) radio + anntena => (broken) radio
# (broken) radio + blue transistor => radio
# (broken) processor + cache => processor
# (broken) motherboard + screw => (broken) motherboard
# (broken) A-1920-IXB + processor => (broken) A-1920-IXB

.PHONY: all clean
