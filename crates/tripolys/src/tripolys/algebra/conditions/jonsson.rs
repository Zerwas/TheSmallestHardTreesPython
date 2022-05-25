use itertools::Itertools;

use crate::digraph::traits::Vertices;

use super::{Arity, Linear, Partition, Precolor};

pub struct Jonsson(pub usize);

impl Precolor for Jonsson {}

impl Linear for Jonsson {
    fn arities(&self) -> Vec<Arity> {
        vec![3; 2 * self.0 + 1]
    }

    fn partition<G>(&self, g: &G) -> Partition<(usize, Vec<G::Vertex>)>
    where
        for<'a> G: Vertices<'a>,
    {
        let mut partition = Vec::new();

        for x in g.vertices() {
            let mut id = (0..(2 * self.0 + 1))
                .map(|i| (i, vec![x, x, x]))
                .collect_vec();

            for y in g.vertices() {
                if x == y {
                    continue;
                }
                for i in 0..self.0 {
                    partition.push(vec![(2 * i, vec![x, y, y]), (2 * i + 1, vec![x, y, y])]);
                    partition.push(vec![(2 * i + 1, vec![x, x, y]), (2 * i + 2, vec![x, x, y])]);
                }
                for i in 0..(2 * self.0 + 1) {
                    id.push((i, vec![x, y, x]));
                }
                partition.push(vec![(0, vec![x, x, x]), (0, vec![x, x, y])]);
                partition.push(vec![
                    (2 * self.0, vec![y, y, y]),
                    (2 * self.0, vec![x, y, y]),
                ]);
            }
            partition.push(id);
        }

        partition
    }
}
