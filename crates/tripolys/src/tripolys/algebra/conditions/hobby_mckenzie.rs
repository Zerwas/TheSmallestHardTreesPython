use itertools::Itertools;

use super::{Arity, Linear, Partition, Precolor, Set, Tuple, Vertex};

pub struct HobbyMcKenzie(pub usize);

impl Precolor for HobbyMcKenzie {
    fn precolor<V: PartialEq + Copy>(&self, (f, v): &(usize, Tuple<V>)) -> Option<V> {
        if *f == 0 {
            return Some(v[0]);
        }
        if *f == (2 * self.0 + 2) {
            return Some(v[2]);
        }
        None
    }
}

impl Linear for HobbyMcKenzie {
    fn arities(&self) -> Vec<Arity> {
        vec![3; 2 * self.0 + 3]
    }

    fn partition<V: Vertex>(&self, set: Set<V>) -> Partition<(usize, Tuple<V>)> {
        let mut partition = Vec::new();

        for &x in &set {
            partition.push(
                (0..(2 * self.0 + 3))
                    .map(|i| (i, vec![x, x, x]))
                    .collect_vec(),
            );

            for &y in &set {
                if x == y {
                    continue;
                }
                partition.push(vec![
                    (self.0 + 0, vec![x, y, y]),
                    (self.0 + 1, vec![x, y, y]),
                ]);
                partition.push(vec![
                    (self.0 + 1, vec![x, x, y]),
                    (self.0 + 2, vec![x, x, y]),
                ]);

                for (d, i) in (0..self.0).step_by(2).cartesian_product([0, self.0 + 2]) {
                    partition.push(vec![(i + d, vec![x, y, y]), (i + d + 1, vec![x, y, y])]);
                }
                for (d, i) in (0..self.0)
                    .skip(1)
                    .step_by(2)
                    .cartesian_product([0, self.0 + 2])
                {
                    partition.push(vec![(i + d, vec![x, y, y]), (i + d + 1, vec![x, y, y])]);
                    partition.push(vec![(i + d, vec![x, y, x]), (i + d + 1, vec![x, y, x])]);
                }
            }
        }

        partition
    }
}
