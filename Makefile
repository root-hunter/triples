# Makefile for building and running examples

# Benchmark Example
benchmark-clean:
	cargo clean --package benchmark

benchmark-build:
	cargo build --release --package benchmark

benchmark-run:
	cargo run --release --package benchmark

benchmark: benchmark-build benchmark-run

# Big Limits Example
big-limits-clean:
	cargo clean --package big-limits

big-limits-build:
	cargo build --release --package big-limits

big-limits-run:
	cargo run --release --package big-limits

big-limits: big-limits-build big-limits-run

# Print Triples Example
print-triples-clean:
	cargo clean --package print-triples

print-triples-build:
	cargo build --release --package print-triples

print-triples-run:
	cargo run --release --package print-triples

print-triples: print-triples-build print-triples-run

# Save on File Example
save-on-file-clean:
	cargo clean --package save-on-file

save-on-file-build:
	cargo build --release --package save-on-file

save-on-file-run: 
	cargo run --release --package save-on-file

save-on-file: save-on-file-build save-on-file-run