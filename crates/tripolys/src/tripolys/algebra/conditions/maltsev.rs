use crate::digraph::traits::Vertices;

use super::{Arity, Operation, Partition, Precolor};

pub struct Maltsev;

impl Precolor for Maltsev {
    fn precolor<V>(&self, _: &(usize, Vec<V>)) -> Option<V> {
        todo!()
    }
}

impl Operation for Maltsev {
    fn arity(&self) -> Arity {
        3
    }

    fn partition<G>(&self, _: &G) -> Partition<Vec<G::Vertex>>
    where
        for<'a> G: Vertices<'a>,
    {
        todo!()
    }
}
