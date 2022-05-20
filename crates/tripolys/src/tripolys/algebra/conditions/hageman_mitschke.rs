use super::{Arity, Linear, Partition, Precolor, Set, Tuple};

pub struct HagemanMitschke(pub usize);

impl Precolor for HagemanMitschke {
    fn precolor<V: PartialEq + Copy>(&self, (f, v): &(usize, Tuple<V>)) -> Option<V> {
        if *f == 0 {
            if v[1] == v[2] {
                return Some(v[0]);
            }
        }
        if *f == (self.0 - 1) {
            if v[0] == v[1] {
                return Some(v[2]);
            }
        }
        None
    }
}

impl Linear for HagemanMitschke {
    fn arities(&self) -> Vec<Arity> {
        vec![3; self.0]
    }

    fn partition<V: Copy>(&self, vertices: Set<V>) -> Partition<(usize, Tuple<V>)> {
        let mut partition = Vec::new();

        for &x in &vertices {
            for &y in &vertices {
                for i in 0..self.0 {
                    partition.push(vec![(i, vec![x, x, y]), (i + 1, vec![x, y, y])]);
                }
            }
        }

        partition
    }
}
