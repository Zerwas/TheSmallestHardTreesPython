use std::iter::FromIterator;

use crate::domains::DomMap;
use crate::problem::Constraints;

use super::revise;

/// The AC-1 algorithm due to Mackworth 1977.
pub fn ac_1(domains: &mut DomMap, constraints: &impl Constraints) -> bool {
    let mut changed = true;

    while changed {
        changed = false;

        for (x, y) in constraints.arcs() {
            if let Some(res) = revise(domains, constraints, x, y) {
                if res {
                    changed = true;
                }
            } else {
                return false;
            }
        }
    }
    true
}

/// The AC-3 algorithm due to Mackworth 1977.
pub fn ac_3(domains: &mut DomMap, constraints: &impl Constraints) -> bool {
    let mut work_list = Vec::from_iter(constraints.arcs());
    let mut neighbors = vec![Vec::new(); domains.vars_count()];

    for arc in constraints.arcs() {
        neighbors[*arc.1].push(arc);
    }

    while let Some((x, y)) = work_list.pop() {
        if let Some(res) = revise(domains, constraints, x, y) {
            if res {
                work_list.extend(neighbors[*x].iter().copied());
            }
        } else {
            return false;
        }
    }
    true
}
