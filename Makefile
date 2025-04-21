num_threads=4
exe_name="target/release/mpi_sorting"

run:
	cargo build --release && mpiexec -n $(num_threads) $(exe_name)
