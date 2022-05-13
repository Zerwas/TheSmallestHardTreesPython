use arx::problem::{Constraints, Domains, Problem, Value, Variable};

use std::hash::Hash;

use crate::algebra::Edges;
use crate::digraph::{AdjMatrix, AdjMap, VertexId};

/// An instance of the H-Colouring problem
#[derive(Debug, Clone)]
pub struct ColouringProblem<V0, V1> {
    pub(crate) domains: Vec<Vec<usize>>,
    pub(crate) g: AdjMatrix<V0>,
    pub(crate) h: AdjMatrix<V1>,
}

impl<V0, V1> ColouringProblem<V0, V1>
where
    V0: VertexId,
    V1: VertexId,
{
    pub fn new(g: &AdjMap<V0>, h: &AdjMap<V1>) -> ColouringProblem<V0, V1>
    {
        let g = g.to_matrix();
        let h = h.to_matrix();
        let mut domains = Vec::with_capacity(g.vertex_count());

        for _ in 0..g.vertex_count() {
            domains.push(Vec::from_iter(h.vertices()));
        }

        ColouringProblem { domains, g, h }
    }
}

impl<V0, V1> ColouringProblem<V0, V1>
where
    V0: Clone + Eq + Hash,
    V1: Clone + Eq + Hash,
{
    pub fn precolour<FN: Fn(&V0) -> Option<V1>>(&mut self, c: FN) {
        for v in self.g.values() {
            if let Some(w) = c(v) {
                self.domains[self.g.index_of(v).unwrap()] = vec![self.h.index_of(&w).unwrap()];
            }
        }
    }
}

impl<V0, V1> Constraints for ColouringProblem<V0, V1> {
    fn arcs(&self) -> Vec<(Variable, Variable)> {
        self.g
            .edges()
            .into_iter()
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

impl<V0, V1> Domains for ColouringProblem<V0, V1> {
    fn size(&self) -> usize {
        self.g.vertex_count()
    }

    fn domain(&self, x: Variable) -> Vec<Value> {
        self.domains[*x].iter().map(|v| Value(*v)).collect()
    }
}

impl<V0, V1> Problem for ColouringProblem<V0, V1> {}
