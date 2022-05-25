use crate::digraph::traits::Vertices;
use arx::problem::{Constraints, Domains, Problem, Value, Variable};

use std::fmt::Debug;

use crate::digraph::traits::Edges;
use crate::digraph::AdjMatrix;

/// An instance of the H-Colouring problem
#[derive(Clone, Debug)]
pub struct Instance {
    domains: Vec<Vec<usize>>,
    g: AdjMatrix,
    h: AdjMatrix,
}

impl Instance {
    pub fn new(g: AdjMatrix, h: AdjMatrix) -> Instance {
        let domains = (0..g.vertex_count())
            .map(|_| Vec::from_iter(h.vertices()))
            .collect();

        Instance { domains, g, h }
    }

    pub fn with_precoloring<FP>(g: AdjMatrix, h: AdjMatrix, c: FP) -> Instance
    where
        FP: Fn(usize) -> Option<usize>,
    {
        let domains = g
            .vertices()
            .map(|v| c(v).map_or(Vec::from_iter(h.vertices()), |v| vec![v]))
            .collect();

        Instance { domains, g, h }
    }

    pub fn with_lists<FL>(g: AdjMatrix, h: AdjMatrix, l: FL) -> Instance
    where
        FL: Fn(usize) -> Vec<usize>,
    {
        let domains = g.vertices().map(|v| l(v)).collect();

        Instance { domains, g, h }
    }
}

impl Constraints for Instance {
    fn arcs(&self) -> Vec<(Variable, Variable)> {
        self.g
            .edges()
            .flat_map(|(u, v)| [(Variable(u), Variable(v)), (Variable(v), Variable(u))])
            .collect()
    }

    fn check(&self, (i, ai): (Variable, Value), (j, aj): (Variable, Value)) -> bool {
        if self.g.has_edge(*i, *j) {
            return self.h.has_edge(*ai, *aj);
        }
        if self.g.has_edge(*j, *i) {
            return self.h.has_edge(*aj, *ai);
        }
        true
    }
}

impl Domains for Instance {
    fn size(&self) -> usize {
        self.g.vertex_count()
    }

    fn domain(&self, x: Variable) -> Vec<Value> {
        self.domains[*x].iter().map(|v| Value(*v)).collect()
    }
}

impl Problem for Instance {}
