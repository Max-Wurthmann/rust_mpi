use mpi::datatype::{Partition, PartitionMut};
use mpi::traits::*;
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
    let displs: Vec<i32> = (0..size).collect();

    let send_partition = Partition::new(&send_buf, counts.clone(), displs.clone());

    counts.iter().zip(displs.iter()).all(|(a, b)| a + b <= size);

    // let test: bool = counts
    //     .borrow()
    //     .iter()
    //     .zip(displs.borrow().iter())
    //     .all(|(&c, &d)| c + d <= n);

    let mut recv_buf: Vec<i32> = Vec::with_capacity(n_threads);
    println!("counts {:?}, displs: {:?}", counts, displs);
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
