use crate::digraph::traits::Vertices;

use super::{Arity, Linear, Partition, Precolor};

/// p(x,y,y) = q(y,x,x) = q(x,x,y), p(x,y,x) = q(x,y,x)
pub struct Kmm;

impl Precolor for Kmm {}

impl Linear for Kmm {
    fn arities(&self) -> Vec<Arity> {
        vec![3, 3]
    }

    fn partition<G>(&self, g: &G) -> Partition<(usize, Vec<G::Vertex>)>
    where
        for<'a> G: Vertices<'a>,
    {
        let mut partition = Vec::new();

        for x in g.vertices() {
            for y in g.vertices() {
                if x == y {
                    partition.push(vec![(0, vec![x, x, x]), (1, vec![x, x, x])]);
                }
                partition.push(vec![
                    (0, vec![x, y, y]),
                    (1, vec![y, x, x]),
                    (1, vec![x, x, y]),
                ]);
                partition.push(vec![(0, vec![x, y, x]), (1, vec![x, y, x])]);
            }
        }

        partition
    }
}
