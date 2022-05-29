use itertools::Itertools;

use crate::digraph::traits::Vertices;

use super::{Arity, Condition, Partition, Precolor};

pub struct KearnesKiss {
    length: usize,
}

pub fn kk(length: usize) -> KearnesKiss {
    KearnesKiss { length }
}

impl Precolor for KearnesKiss {}

impl Condition for KearnesKiss {
    fn arities(&self) -> Vec<Arity> {
        vec![3; self.length + 1]
    }

    fn partition<G>(&self, g: &G) -> Partition<(usize, Vec<G::Vertex>)>
    where
        for<'a> G: Vertices<'a>,
    {
        let mut partition = Vec::new();

        for x in g.vertices() {
            let mut id = (0..=self.length).map(|i| (i, vec![x, x, x])).collect_vec();

            for y in g.vertices() {
                for i in (0..self.length).step_by(2) {
                    partition.push(vec![(i, vec![x, y, y]), (i + 1, vec![x, y, y])]);
                    partition.push(vec![(i, vec![x, y, x]), (i + 1, vec![x, y, x])]);
                }
                for i in (0..self.length).skip(1).step_by(2) {
                    partition.push(vec![(i, vec![x, x, y]), (i + 1, vec![x, x, y])]);
                }
                for z in g.vertices() {
                    id.push((0, vec![x, y, z]));
                    id.push((self.length, vec![y, z, x]));
                }
            }
            partition.push(id);
        }

        partition
    }
}
