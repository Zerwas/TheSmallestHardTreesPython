use crate::hcoloring::Instance;

use super::traits::{Digraph, GraphType};
use super::{classes::directed_path, AdjMatrix};
use arx::solver::BTSolver;

pub fn levels<'a, G>(g: &'a G) -> Option<Vec<usize>>
where
    G: Digraph<'a> + GraphType<Vertex = usize>,
{
    for k in 0..g.vertex_count() {
        let g = AdjMatrix::build_from(g);
        let h: AdjMatrix = directed_path(k + 1);
        let problem = Instance::new(g, h);

        if let Some(sol) = BTSolver::new(&problem).solve_first() {
            return Some(sol.into_iter().map(|v| *v).collect());
        }
    }
    None
}
