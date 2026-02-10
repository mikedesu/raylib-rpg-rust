all:
	cargo build
	./log_build_stats.sh >> build_stats.csv

build:
	#cargo build --release
	cargo build 
	./log_build_stats.sh >> build_stats.csv

run:
	#cargo run --release
	cargo run 
	./log_build_stats.sh >> build_stats.csv

clean:
	cargo clean
