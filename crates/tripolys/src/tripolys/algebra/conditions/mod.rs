//! Identities that are satisfied by polymorphisms.

mod hageman_mitschke;
mod hobby_mckenzie;
mod jonsson;
mod kearnes_kiss;
mod kmm;
mod maltsev;
mod near_unamity;
mod noname;
mod siggers;
mod sigma;

pub use hageman_mitschke::*;
pub use hobby_mckenzie::*;
pub use jonsson::*;
pub use kearnes_kiss::*;
pub use kmm::*;
pub use near_unamity::*;
pub use noname::*;
pub use siggers::*;

use std::hash::Hash;

use crate::digraph::traits::Vertices;

pub type Arity = usize;
// pub type Set<V> = Vec<V>;
pub type Partition<V> = Vec<Vec<V>>;

pub trait Operation {
    fn arity(&self) -> Arity;

    fn partition<G>(&self, vertices: &G) -> Partition<Vec<G::Vertex>>
    where
        for<'a> G: Vertices<'a>;
}

impl<O: Operation + Precolor> Condition for O {
    fn arities(&self) -> Vec<Arity> {
        vec![self.arity()]
    }

    fn partition<G>(&self, _vertices: &G) -> Partition<(usize, Vec<G::Vertex>)>
    where
        for<'a> G: Vertices<'a>,
    {
        self.partition(_vertices)
            .into_iter()
            .map(|v| v.into_iter().map(|t| (0, t)).collect())
            .collect()
    }
}

pub trait H1 {}

pub trait Precolor {
    fn precolor<V: Copy + Eq + Hash>(&self, _: &(usize, Vec<V>)) -> Option<V> {
        None
    }
}

pub trait Condition: Precolor {
    /// The arity for each operation symbol
    fn arities(&self) -> Vec<Arity>;

    /// TODO
    fn partition<G>(&self, vertices: &G) -> Partition<(usize, Vec<G::Vertex>)>
    where
        for<'a> G: Vertices<'a>;
}

/// An identity where each side has exactly one occurrence of an operation symbol.
pub trait HeightOne {
    fn eq_under<V: PartialEq>(t1: &[V], t2: &[V]) -> bool;
}

#[cfg(test)]
mod tests {
    use super::HeightOne;
    use crate::algebra::conditions::*;

    #[test]
    fn test_height1() {
        fn test<C: Condition + HeightOne>(_condition: C) {
            // for set in condition.partition(&[0, 1, 2]) {
            //     assert!(set.windows(2).all(|w| condition.eq_under(&w[0], &w[1])));
            // }
        }

        // test();
        // test(Sigma(2));
        // test(Sigma(3));
        // test(Wnu(3));
        // test(Wnu(4));
        // test(Nu(3));
        // test(Nu(4));
    }
}
