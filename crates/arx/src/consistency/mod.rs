//! A set of consistency algorithms.

mod ac;
mod sac;
// mod pc;

pub use ac::ac_1;
pub use ac::ac_3;
pub use sac::sac_1;

use crate::domains::DomMap;
use crate::problem::{Constraints, Variable};

fn revise<C>(domains: &mut DomMap, constraints: &C, x: Variable, y: Variable) -> Option<bool>
where
    C: Constraints,
{
    let mut mutated = false;

    for i in domains.indices(x) {
        let mut is_possible = false;

        for j in domains.indices(y) {
            if constraints.check((x, domains.value(x, i)), (y, domains.value(y, j))) {
                is_possible = true;
                break;
            }
        }

        if !is_possible {
            domains.remove(x, i);
            mutated = true;

            if domains.get(x).is_empty() {
                return None;
            }
        }
    }

    Some(mutated)
}
