use crate::digraph::traits::Vertices;

use super::{Arity, Condition, Partition, Precolor};

pub struct NoName {
    length: usize,
}

impl NoName {
    pub fn with_length(n: usize) -> NoName {
        NoName { length: n }
    }
}

impl Precolor for NoName {
    fn precolor<V: PartialEq + Copy>(&self, (f, v): &(usize, Vec<V>)) -> Option<V> {
        if *f == 0 && v[1] == v[2] {
            return Some(v[0]);
        }
        if *f == self.length && v[0] == v[1] {
            return Some(v[3]);
        }
        None
    }
}

impl Condition for NoName {
    fn arities(&self) -> Vec<Arity> {
        vec![4; self.length + 1]
    }

    fn partition<G>(&self, g: &G) -> Partition<(usize, Vec<G::Vertex>)>
    where
        for<'a> G: Vertices<'a>,
    {
        let mut partition = Vec::new();

        for x in g.vertices() {
            for y in g.vertices() {
                for i in 0..self.length {
                    partition.push(vec![(i, vec![x, x, y, x]), (i + 1, vec![x, y, y, x])]);
                    partition.push(vec![(i, vec![x, x, y, y]), (i + 1, vec![x, y, y, y])]);
                }
            }
        }

        partition
    }
}
