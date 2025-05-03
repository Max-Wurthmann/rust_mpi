n ?= 4
exe_name = "target/release/rust_mpi"

run:
	cargo build --release && mpiexec -n $(n) $(exe_name)
