num_threads=4
exe_name="target/release/rust_mpi"

run:
	cargo build --release && mpiexec -n $(num_threads) $(exe_name)
