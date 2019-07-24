# Task00: Implement um and run benchmark

all: download benchmark

download:
	curl -O http://www.boundvariable.org/sandmark.umz

benchmark:
	cargo run --release --bin my-icfp2006-um -- ./sandmark.umz
