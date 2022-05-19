use arx::problem::*;
use itertools::Itertools;

use crate::colouring::ColouringProblem;
use crate::digraph::AdjMap;
use crate::tree::{Tree, Balanced};

use super::conditions::{Linear, Tuple, Vertex};
use super::{Edges, IterAlgebra, Vertices};

/// The problem of deciding whether a graph has a given type of polymorphism(s).
pub struct MetaProblem<V> {
    problem: ColouringProblem<(usize, Tuple<V>), V>,
}

impl<V: Vertex> MetaProblem<V> {
    pub fn new<H, C>(h: &H, condition: C) -> MetaProblem<V>
    where
        H: Vertices<V> + Edges<V>,
        C: Linear,
    {
        let mut indicator = condition
            .arities()
            .into_iter()
            .enumerate()
            .flat_map(|(i, k)| h.edge_iter().power(k).map(move |(u, v)| ((i, u), (i, v))))
            .collect::<AdjMap<_>>();

        for set in condition.partition(h.vertex_iter().collect::<Vec<_>>()) {
            // println!("set: {:?}", set);
            for i in 1..set.len() {
                indicator.contract_vertices(&set[0], &set[i]);
            }
        }

        let mut problem = ColouringProblem::new(&indicator, &AdjMap::from_iter(h.edge_iter()));
        problem.precolour(|v| condition.precolor(v));

        MetaProblem { problem }
    }

    pub fn from_tree<T, C>(t: &T, condition: C) -> MetaProblem<u32>
    where
        T: Tree + Balanced,
        C: Linear,
    {
        let h = t.to_graph();

        let mut indicator = condition
            .arities()
            .into_iter()
            .enumerate()
            .flat_map(|(i, k)| h.edge_iter().power(k).map(move |(u, v)| ((i, u), (i, v))))
            .filter(|((_, u), _)| u.iter().map(|v| t.level(v).unwrap()).all_equal())
            .collect::<AdjMap<_>>();

        for set in condition.partition(h.vertex_iter().collect::<Vec<_>>()) {
            // println!("set: {:?}", set);
            for i in 1..set.len() {
                indicator.contract_vertices(&set[0], &set[i]);
            }
        }

        let mut problem = ColouringProblem::new(&indicator, &AdjMap::from_iter(h.edges()));
        problem.precolour(|v| condition.precolor(v));

        MetaProblem { problem }
    }
}

impl<V: Vertex> MetaProblem<V> {
    pub fn conservative(&mut self) {
        for v in self.problem.g.values() {
            self.problem.domains[self.problem.g.index_of(v).unwrap()] =
                v.1.iter()
                    .map(|s| self.problem.h.index_of(s).unwrap())
                    .collect();
        }
    }

    pub fn idempotent(&mut self) {
        for v in self.problem.g.values() {
            if v.1.iter().all_equal() {
                self.problem.domains[self.problem.g.index_of(v).unwrap()] =
                    vec![self.problem.h.index_of(&v.1[0]).unwrap()];
            }
        }
    }
}

impl<V> Constraints for MetaProblem<V> {
    fn arcs(&self) -> Vec<(Variable, Variable)> {
        self.problem.arcs()
    }

    fn check(&self, ai: (Variable, Value), aj: (Variable, Value)) -> bool {
        self.problem.check(ai, aj)
    }
}

impl<V> Domains for MetaProblem<V> {
    fn size(&self) -> usize {
        self.problem.size()
    }

    fn domain(&self, x: Variable) -> Vec<Value> {
        self.problem.domain(x)
    }
}

impl<V> Problem for MetaProblem<V> {}
