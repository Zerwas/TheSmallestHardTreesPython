use itertools::Itertools;

use crate::digraph::traits::Vertices;

use super::{Arity, Condition, Partition, Precolor};

pub struct HobbyMcKenzie {
    length: usize,
}

impl HobbyMcKenzie {
    pub fn with_length(n: usize) -> HobbyMcKenzie {
        HobbyMcKenzie { length: n }
    }
}

impl Precolor for HobbyMcKenzie {
    fn precolor<V: Copy + PartialEq>(&self, (f, v): &(usize, Vec<V>)) -> Option<V> {
        if *f == 0 {
            return Some(v[0]);
        }
        if *f == (2 * self.length + 2) {
            return Some(v[2]);
        }
        None
    }
}

impl Condition for HobbyMcKenzie {
    fn arities(&self) -> Vec<Arity> {
        vec![3; 2 * self.length + 3]
    }

    #[allow(clippy::identity_op)]
    fn partition<G>(&self, g: &G) -> Partition<(usize, Vec<G::Vertex>)>
    where
        for<'a> G: Vertices<'a>,
    {
        let mut partition = Vec::new();

        for x in g.vertices() {
            partition.push(
                (0..(2 * self.length + 3))
                    .map(|i| (i, vec![x, x, x]))
                    .collect_vec(),
            );

            for y in g.vertices() {
                if x == y {
                    continue;
                }
                partition.push(vec![
                    (self.length + 0, vec![x, y, y]),
                    (self.length + 1, vec![x, y, y]),
                ]);
                partition.push(vec![
                    (self.length + 1, vec![x, x, y]),
                    (self.length + 2, vec![x, x, y]),
                ]);

                for (d, i) in (0..self.length)
                    .step_by(2)
                    .cartesian_product([0, self.length + 2])
                {
                    partition.push(vec![(i + d, vec![x, y, y]), (i + d + 1, vec![x, y, y])]);
                }
                for (d, i) in (0..self.length)
                    .skip(1)
                    .step_by(2)
                    .cartesian_product([0, self.length + 2])
                {
                    partition.push(vec![(i + d, vec![x, y, y]), (i + d + 1, vec![x, y, y])]);
                    partition.push(vec![(i + d, vec![x, y, x]), (i + d + 1, vec![x, y, x])]);
                }
            }
        }

        partition
    }
}
