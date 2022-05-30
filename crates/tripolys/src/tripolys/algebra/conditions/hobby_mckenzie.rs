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

    fn partition<G>(&self, g: &G) -> Partition<(usize, Vec<G::Vertex>)>
    where
        for<'a> G: Vertices<'a>,
    {
        let n = self.length;
        let mut partition = Vec::new();

        for x in g.vertices() {
            partition.push((0..(2 * n + 3)).map(|i| (i, vec![x, x, x])).collect_vec());

            for y in g.vertices() {
                if x == y {
                    continue;
                }
                partition.push(vec![(n, vec![x, y, y]), (n + 1, vec![x, y, y])]);
                partition.push(vec![(n + 1, vec![x, x, y]), (n + 2, vec![x, x, y])]);

                for j in (0..n).step_by(2) {
                    partition.push(vec![(j, vec![x, y, y]), (j + 1, vec![x, y, y])]);
                    partition.push(vec![(j + n + 2, vec![x, y, y]), (j + n + 3, vec![x, y, y])]);
                    partition.push(vec![(j + n + 2, vec![x, y, x]), (j + n + 3, vec![x, y, x])]);
                }
                for j in (0..n).skip(1).step_by(2) {
                    partition.push(vec![(j, vec![x, x, y]), (j + 1, vec![x, x, y])]);
                    partition.push(vec![(j, vec![x, y, x]), (j + 1, vec![x, y, x])]);
                    partition.push(vec![(j + n + 2, vec![x, x, y]), (j + n + 3, vec![x, x, y])]);
                }
            }
        }

        partition
    }
}
