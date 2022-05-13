//! Identities that are satisfied by polymorphisms.

// mod custom;
mod jonsson;
mod kiss_kearnes;
mod kmm;
mod near_unamity;
mod noname;
mod siggers;
mod sigma;
mod maltsev;
mod hageman_mitschke;

pub use jonsson::Jonsson;
pub use kiss_kearnes::KearnesKiss;
pub use kmm::Kmm;
pub use near_unamity::{Majority, Nu, Wnu};
pub use noname::Noname;
pub use siggers::Siggers;
pub use sigma::Sigma;

use std::fmt::Debug;
use std::hash::Hash;

pub trait Vertex: Copy + Hash + Eq + Debug + Send + Sync {}

impl<V> Vertex for V where V: Copy + Hash + Eq + Debug + Send + Sync {}

pub type Arity = usize;
pub type Set<V> = Vec<V>;
pub type Partition<V> = Set<Set<V>>;
pub type Tuple<V> = Vec<V>;

pub trait Operation {
    fn arity(&self) -> Arity;

    fn partition<V: Vertex>(&self, vertices: Set<V>) -> Partition<Tuple<V>>;
}

impl<O: Operation + Precolor> Linear for O {
    fn arities(&self) -> Vec<Arity> {
        vec![self.arity()]
    }

    fn partition<V: Vertex>(&self, vertices: Set<V>) -> Partition<(usize, Tuple<V>)> {
        self.partition(vertices)
            .into_iter()
            .map(|v| v.into_iter().map(|t| (0, t)).collect())
            .collect()
    }
}

pub trait H1 {}

pub trait Precolor {
    fn precolor<V: Vertex>(&self, v: &(usize, Tuple<V>)) -> Option<V> {
        None
    }
}

pub trait Linear: Precolor {
    /// The arity for each operation symbol
    fn arities(&self) -> Vec<Arity>;

    /// TODO
    fn partition<V: Vertex>(&self, vertices: Set<V>) -> Partition<(usize, Tuple<V>)>;
}

/// An identity where each side has exactly one occurrence of an operation symbol.
pub trait HeightOne {
    fn eq_under<V: Vertex>(t1: &[V], t2: &[V]) -> bool;
}

#[cfg(test)]
mod tests {
    use super::HeightOne;
    use crate::algebra::conditions::*;

    #[test]
    fn test_height1() {
        fn test<C: Linear + HeightOne>(condition: C) {
            // for set in condition.partition(&[0, 1, 2]) {
            //     assert!(set.windows(2).all(|w| condition.eq_under(&w[0], &w[1])));
            // }
        }

        test(Siggers);
        test(Sigma(2));
        test(Sigma(3));
        test(Wnu(3));
        test(Wnu(4));
        test(Nu(3));
        test(Nu(4));
    }
}
