//! A connected, acyclic digraph.
pub mod generate;
mod node;

pub use node::Tree;

use arx::consistency::ac_3;
use arx::domains::DomMap;

use crate::digraph::traits::{Digraph, GraphType};
use crate::digraph::AdjMatrix;
use crate::hcoloring::Instance;

/// A rooted tree is a tree in which one vertex has been designated the root.
pub trait Rooted<'a>: Digraph<'a> {
    fn root(&self) -> Self::Vertex;
}

pub fn is_core_tree<'a, T>(t: &'a T) -> bool
where
    T: Digraph<'a> + GraphType<Vertex = usize>,
{
    let mat = AdjMatrix::build_from(t);
    let problem = Instance::new(mat.clone(), mat);
    let mut domains = DomMap::new(&problem);
    let _ = ac_3(&mut domains, &problem);

    for x in domains.vars() {
        if domains.get(x).size() != 1 {
            return false;
        }
    }

    true
}

// /// A digraph H is balanced if its vertices can be organized into levels, that
// /// is, there exists a function lvl : H → N such that lvl(v) = lvl(u) + 1 for
// /// all (u, v) ∈ E(H) and the smallest level is 0. The height of H is the
// /// maximum level.
// pub trait Balanced {
//     /// Returns None, if the tree has no vertex with id `id`. Otherwise, the
//     /// rank of the respective vertex is returned.
//     fn level(&self, id: &u32) -> Option<usize>;
// }

pub fn is_rooted_core_tree<'a, R>(rt: &'a R) -> bool
where
    R: Rooted<'a> + GraphType<Vertex = usize>,
{
    let colouring = move |v: usize| {
        if v == rt.root() {
            Some(rt.root())
        } else {
            None
        }
    };

    let mat = AdjMatrix::build_from(rt);
    let problem = Instance::with_precoloring(mat.clone(), mat, colouring);
    let mut domains = DomMap::new(&problem);
    let _ = ac_3(&mut domains, &problem);

    for x in domains.vars() {
        if domains.get(x).size() != 1 {
            return false;
        }
    }
    true
}
