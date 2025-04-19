num_threads=4

run:
	cargo build --release
	mpiexec -n $(num_threads) target/release/sorting
