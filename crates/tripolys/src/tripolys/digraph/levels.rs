use arx::solver::BTSolver;

use crate::colouring::ColouringProblem;

use super::{AdjMap, adj_map::directed_path};

pub fn levels(g: &AdjMap<u32>) -> Option<Vec<usize>> {
    for k in 0..g.vertex_count() {
        let problem = ColouringProblem::new(g, &directed_path((k + 1) as u32));

        if let Some(sol) = BTSolver::new(&problem).solve_first() {
            return Some(sol.into_iter().map(|v| *v).collect());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    // use super::*;

    // //TODO
}

// pub fn levels(g: &AdjMap<usize>) -> Result<Vec<usize>, usize> {
//     let mut levels = vec![None; g.vertex_count()];
//     let mut roots = g
//         .vertices()
//         .copied()
//         .filter(|v| g.in_degree(v) == 0)
//         .inspect(|&v| levels[v] = Some(0))
//         .collect_vec();

//     for k in 0..levels.len() {
//         for v in &roots {
//             for n in g.out_neighbors(v) {
//                 if let Some(l) = levels[*n] {
//                     if k != l {}
//                 }
//             }
//         }
//     }
//     // let mut marked = HashSet::with_capacity(roots.len());
//     Ok(levels.into_iter().map(|l| l.unwrap()).collect())
// }

// fn visit<N, FN, IN>(
//     node: &N,
//     successors: &mut FN,
//     unmarked: &mut HashSet<N>,
//     marked: &mut HashSet<N>,
//     temp: &mut HashSet<N>,
//     sorted: &mut VecDeque<N>,
// ) -> Result<(), N>
// where
//     N: Eq + Hash + Clone,
//     FN: FnMut(&N) -> IN,
//     IN: IntoIterator<Item = N>,
// {
//     unmarked.remove(node);
//     if marked.contains(node) {
//         return Ok(());
//     }
//     if temp.contains(node) {
//         return Err(node.clone());
//     }
//     temp.insert(node.clone());
//     for n in successors(node) {
//         visit(&n, successors, unmarked, marked, temp, sorted)?;
//     }
//     marked.insert(node.clone());
//     sorted.push_front(node.clone());
//     Ok(())
// }
