test-old:
	gcc -o old_code/main old_code/main.c -O3 -lm
	time ./old_code/main 100000

test-new:
	cargo build --release
	time ./target/release/black