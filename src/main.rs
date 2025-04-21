use mpi::{
    datatype::{Partition, PartitionMut},
    traits::*,
};
use std::iter;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();
    let size = world.size();

    println!("{}", size);

    let n_threads = 4;

    let send_buf: Vec<i32> = iter::repeat(rank).take(size as usize).collect();
    let counts: Vec<i32> = iter::repeat(1).take(size as usize).collect();
    let displs: Vec<i32> = (0..3).collect();

    let send_partition = Partition::new(&send_buf, counts.clone(), displs.clone());

    let mut recv_buf: Vec<i32> = Vec::with_capacity(n_threads);
    let mut recv_partiotion = PartitionMut::new(&mut recv_buf, counts, displs);

    world
        .this_process()
        .all_to_all_varcount_into(&send_partition, &mut recv_partiotion);

    // let mut recv_buf = vec![0_i32; chunk_len];
    // if rank == root_rank {
    //     let send_buf: Vec<i32> = (0..(size * chunk_len as i32)).collect();
    //     root_process.scatter_into_root(&send_buf, &mut recv_buf);
    // } else {
    //     root_process.scatter_into(&mut recv_buf);
    // }

    println!("rank {rank} reports: {:?}", recv_buf);
}
