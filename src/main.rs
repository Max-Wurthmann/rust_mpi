use mpi::datatype::{Partition, PartitionMut};
use mpi::traits::*;
use rust_mpi::{mpi_alltoallv, mpi_scatter};

fn main() {
    mpi_scatter();
    mpi_alltoallv();

    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();
    let size = world.size();

    println!("{}", size);

    let counts: Vec<i32> = vec![1; size as usize];
    let displs: Vec<i32> = (0..size).collect();

    let send_buf: Vec<i32> = (0..size).collect();
    let send_partition = Partition::new(&send_buf, counts.clone(), displs.clone());

    let mut recv_buf: Vec<i32> = vec![-1; size as usize];
    let mut recv_partiotion = PartitionMut::new(&mut recv_buf, counts, displs);

    world
        .any_process()
        .all_to_all_varcount_into(&send_partition, &mut recv_partiotion);

    println!("rank {rank} reports: {:?}", recv_buf);
}
