all:
	cargo build
	./log_build_stats.sh

build:
	cargo build --release

run:
	cargo run --release

clean:
	cargo clean
