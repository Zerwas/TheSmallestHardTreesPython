use super::{Arity, Linear, Partition, Set, Tuple, Precolor};

pub struct Noname(pub usize);

impl Precolor for Noname {}

impl Linear for Noname {
    fn arities(&self) -> Vec<Arity> {
        vec![3; self.0 + 1]
    }

    fn partition<V>(&self, vertices: Set<V>) -> Partition<(usize, Tuple<V>)> {
        todo!()
    }
}
