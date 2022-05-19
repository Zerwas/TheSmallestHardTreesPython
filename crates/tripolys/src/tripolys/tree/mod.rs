//! A connected, acyclic digraph.
pub mod generate;
mod node;
mod triad;

pub use node::Node;
pub use triad::Triad;

use arx::consistency::ac_3;
use arx::domains::DomMap;

use crate::colouring::ColouringProblem;
use crate::digraph::ToGraph;

pub trait Tree: ToGraph<V = u32> + Balanced {}

/// A rooted tree is a tree in which one vertex has been designated the root.
pub trait Rooted {
    fn root(&self) -> u32 {
        0
    }
}

pub fn is_core_tree<T: Tree>(t: &T) -> bool {
    let problem = ColouringProblem::new(&t.to_graph(), &t.to_graph());
    let mut domains = DomMap::new(&problem);
    let _ = ac_3(&mut domains, &problem);

    for x in domains.vars() {
        if domains.get(x).size() != 1 {
            return false;
        }
    }

    true
}

/// A digraph H is balanced if its vertices can be organized into levels, that
/// is, there exists a function lvl : H → N such that lvl(v) = lvl(u) + 1 for
/// all (u, v) ∈ E(H) and the smallest level is 0. The height of H is the
/// maximum level.
pub trait Balanced {
    /// Returns None, if the tree has no vertex with id `id`. Otherwise, the
    /// rank of the respective vertex is returned.
    fn level(&self, id: &u32) -> Option<usize>;
}

pub fn is_rooted_core_tree<R: Rooted + Tree>(rooted_tree: &R) -> bool {
    let graph = rooted_tree.to_graph();
    let colouring = move |v: &u32| {
        if *v == rooted_tree.root() {
            Some(rooted_tree.root())
        } else {
            None
        }
    };

    let mut problem = ColouringProblem::new(&graph, &graph);
    problem.precolour(colouring);
    let mut domains = DomMap::new(&problem);
    let _ = ac_3(&mut domains, &problem);

    for x in domains.vars() {
        if domains.get(x).size() != 1 {
            return false;
        }
    }
    true
}
