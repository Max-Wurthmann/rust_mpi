use mpi::traits::*;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();
    let size = world.size();

    let root_rank = 0;
    let root_process = world.process_at_rank(root_rank);

    let chunk_len = 5;

    let mut recv_buf = vec![0_i32; chunk_len];
    if rank == root_rank {
        let send_buf: Vec<i32> = (0..(size * chunk_len as i32)).collect();
        root_process.scatter_into_root(&send_buf, &mut recv_buf);
    } else {
        root_process.scatter_into(&mut recv_buf);
    }

    println!("rank {rank} reports: {recv_buf:?}");
}
