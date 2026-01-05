benchmark-clean:
	cargo clean --package benchmark

benchmark-build:
	cargo build --release --package benchmark

benchmark-run:
	cargo run --release --package benchmark