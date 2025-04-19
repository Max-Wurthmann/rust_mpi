use mpi::traits::{Communicator, Root};

pub fn sort(values: &[i64]) -> &[i64] {
    let world = mpi::initialize().unwrap().world();
    let world_size = world.size();
    let rank = world.rank();
    let root_rank = 0;

    assert_eq!(values.len() % world_size as usize, 0);

    let local_size = values.len() / world_size as usize;

    let mut recv_buf: Vec<i32> = Vec::with_capacity(local_size);

    // send chunks to all processes
    if rank == root_rank {
        world
            .this_process()
            .scatter_into_root(values, &mut recv_buf);
    }

    values
}

#[cfg(test)]
mod tests {

    use super::sort;
    use mpi::traits::*;
    use rand::Rng;

    #[test]
    fn sort_correct() {
        let vec_size = 1000 * 4;

        let values: Vec<i64> = rand::rng().random_iter().take(vec_size).collect();
        let mut expected = values.clone();
        expected.sort_unstable();
        let result = sort(&values);
        assert_eq!(expected, result);
    }

    #[test]
    fn mpi_basics() {
        let world = mpi::initialize().unwrap().world();

        let rank = world.rank();
        let size = world.size();

        let root_rank = 0;
        let root_process = world.process_at_rank(root_rank);

        let mut x = 0;

        if rank == root_rank {
            let v = (0..size).collect::<Vec<_>>();

            root_process.scatter_into_root(&v, &mut x);
        } else {
            root_process.scatter_into(&mut x);
        }

        println!("size: {size}");
        assert_eq!(x, rank);
    }
}
