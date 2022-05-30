use itertools::Itertools;

use crate::digraph::traits::Vertices;

use super::{Arity, Condition, Partition, Precolor};

pub struct Jonsson {
    length: usize,
}

impl Jonsson {
    pub fn with_length(n: usize) -> Jonsson {
        Jonsson { length: n }
    }
}

impl Precolor for Jonsson {}

impl Condition for Jonsson {
    fn arities(&self) -> Vec<Arity> {
        vec![3; 2 * self.length + 1]
    }

    fn partition<G>(&self, g: &G) -> Partition<(usize, Vec<G::Vertex>)>
    where
        for<'a> G: Vertices<'a>,
    {
        let mut partition = Vec::new();

        for x in g.vertices() {
            let mut id = (0..=(2 * self.length))
                .map(|i| (i, vec![x, x, x]))
                .collect_vec();

            for y in g.vertices() {
                if x == y {
                    continue;
                }
                for i in 0..self.length {
                    partition.push(vec![(2 * i, vec![x, y, y]), (2 * i + 1, vec![x, y, y])]);
                    partition.push(vec![(2 * i + 1, vec![x, x, y]), (2 * i + 2, vec![x, x, y])]);
                }
                for i in 0..=(2 * self.length) {
                    id.push((i, vec![x, y, x]));
                }
                partition.push(vec![(0, vec![x, x, x]), (0, vec![x, x, y])]);
                partition.push(vec![
                    (2 * self.length, vec![y, y, y]),
                    (2 * self.length, vec![x, y, y]),
                ]);
            }
            partition.push(id);
        }

        partition
    }
}
