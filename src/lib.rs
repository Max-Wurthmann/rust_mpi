use mpi::traits::{Communicator, Root};

pub fn sort<'a>(values: &'a [i32], splitter: &[i32]) -> &'a [i32] {
    let world = mpi::initialize().unwrap().world();
    let world_size = world.size();
    let rank = world.rank();

    let root_rank = 0;
    let root_process = world.process_at_rank(root_rank);

    assert_eq!(values.len() % world_size as usize, 0);

    let local_size = values.len() / world_size as usize;

    let mut recv_buf: Vec<i32> = Vec::with_capacity(local_size);

    if rank == root_rank {
        let send_buf: Vec<i32> = (0..(world_size * local_size as i32)).collect();
        root_process.scatter_into_root(&send_buf, &mut recv_buf);
    } else {
        root_process.scatter_into(&mut recv_buf);
    }

    let mut buckets: Vec<Vec<i32>> = Vec::new();
    recv_buf.into_iter().for_each(|x| {
        let idx = match splitter.binary_search(&x) {
            // we dont care weather the searched item is in splitter,
            // we just care which bucket to put it in
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        buckets[idx].push(x);
    });

    todo!()
    // do alltoallv comm to distribute buckets
    // sort locally
}

#[cfg(test)]
mod tests {

    use super::sort;
    use mpi::traits::*;
    use rand::Rng;

    #[test]
    fn sort_correct() {
        let vec_size = 1000 * 4;

        let values: Vec<i32> = rand::rng().random_iter().take(vec_size).collect();
        let mut expected = values.clone();
        expected.sort_unstable();
        let result = sort(&values, &[5, 10, 15]);
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

        assert_eq!(x, rank);
    }
}
