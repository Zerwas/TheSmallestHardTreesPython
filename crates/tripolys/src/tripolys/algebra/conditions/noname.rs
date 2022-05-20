use super::{Arity, Linear, Partition, Precolor, Set, Tuple};

pub struct Noname(pub usize);

impl Precolor for Noname {
    fn precolor<V: PartialEq + Copy>(&self, (f, v): &(usize, Tuple<V>)) -> Option<V> {
        if *f == 0 {
            if v[1] == v[2] {
                return Some(v[0]);
            }
        }
        if *f == self.0 {
            if v[0] == v[1] {
                return Some(v[3]);
            }
        }
        None
    }
}

impl Linear for Noname {
    fn arities(&self) -> Vec<Arity> {
        vec![4; self.0 + 1]
    }

    fn partition<V: Copy>(&self, vertices: Set<V>) -> Partition<(usize, Tuple<V>)> {
        let mut partition = Vec::new();

        for &x in &vertices {
            for &y in &vertices {
                for i in 0..self.0 {
                    partition.push(vec![(i, vec![x, x, y, x]), (i + 1, vec![x, y, y, x])]);
                    partition.push(vec![(i, vec![x, x, y, y]), (i + 1, vec![x, y, y, y])]);
                }
            }
        }

        partition
    }
}
