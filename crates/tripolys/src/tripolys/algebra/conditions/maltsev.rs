use super::{Arity, Linear, Operation, Partition, Precolor, Set, Tuple, Vertex};

pub struct Maltsev;

impl Precolor for Maltsev {
    fn precolor<V: super::Vertex>(&self, v: &(usize, super::Tuple<V>)) -> Option<V> {
        None
    }
}

impl Operation for Maltsev {
    fn arity(&self) -> Arity {
        3
    }

    fn partition<V: Vertex>(&self, vertices: Set<V>) -> Partition<Tuple<V>> {
        todo!()
    }
}
