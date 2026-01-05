# Makefile for building and running examples

# Benchmark Example
benchmark-clean:
	cargo clean --package benchmark

benchmark-build:
	cargo build --release --package benchmark

benchmark-run:
	cargo run --release --package benchmark

benchmark: benchmark-build
	cd target/release && ./benchmark

# Big Limits Example
big-limits-clean:
	cargo clean --package big-limits

big-limits-build:
	cargo build --release --package big-limits

big-limits-run:
	cargo run --release --package big-limits

big-limits: big-limits-build
	cd target/release && ./big-limits