use crate::domains::DomMap;
use crate::problem::Constraints;

/// The SAC-1 algorithm due to Bessiere and Debruyne 1997.
pub fn sac_1<C, AC>(domains: &mut DomMap, constraints: &C, ac: AC) -> bool
where
    C: Constraints,
    AC: Fn(&mut DomMap, &C) -> bool,
{
    if ac(domains, constraints) {
        return false;
    }

    let mut changed = true;

    while changed {
        changed = false;

        for x in domains.vars() {
            for i in domains.indices(x) {
                let mut domains_x_i = domains.clone();
                domains_x_i.set(x, i);

                if !ac(&mut domains_x_i, constraints) {
                    domains.remove(x, i);
                    changed = true;
                };
            }
            if domains.get(x).is_empty() {
                return false;
            }
        }
    }
    true
}

// /// The SAC-Opt algorithm due to Bessiere and Debruyne 2008.
// ///
// /// Returns None, if an empty domain is derived for some vertex v, otherwise
// /// singleton-arc-consistent domains are returned.
// ///
// /// - `domains` represents the domain for each variable v.
// /// - `constraints` is the set of all constraints for each variable pair (V0, V0).
// ///
// pub fn sac_opt<C, AC>(domains: &mut Domains, constraints: &C, ac: AC) -> bool
// where
//     C: Constraints,
//     AC: Fn(&mut Domains, &C) -> bool,
// {
//     if ac(domains, constraints) {
//         return false;
//     }

//     let mut pending_list = Set::new();
//     let mut ds = HashMap::<(X, D), HashMap<X, Vec<D>>>::new();
//     let mut q = HashMap::<(X, D), HashSet<(X, D)>>::new();

//     // Init phase
//     for x in domains.variables() {
//         for a in domains.indices(x) {
//             let mut domains_i_a = domains.clone();
//             domains_i_a.assign(x, a);
//             ds.insert((x, a), domains_i_a);

//             let mut set = HashSet::<(X, D)>::new();
//             for b in dom_i {
//                 if *b != *a {
//                     set.insert((x, b));
//                 }
//             }
//             q.insert((x, a), set);
//             pending_list.insert((x, a));
//         }
//     }

//     // Propag phase
//     while let Some((i, a)) = pending_list.pop() {
//         let d = ds.get_mut(&(i, a)).unwrap();
//         for (x, y) in q.get(&(i, a)).unwrap() {
//             d.remove(x, y);
//         }
//         if let Some(v) = ac(d, constraints) {
//             q.get_mut(&(i, a)).unwrap().clear();
//             *d = v;
//         } else {
//             domains.remove(i, a);
//             if domains.values_count(i) == 0 {
//                 return false;
//             }
//             for ((j, b), m) in &mut ds {
//                 if m.domain_mut(i).unwrap().remove(a) {
//                     q.get_mut(&(j, b)).unwrap().insert((i, a));
//                     pending_list.insert((j, b));
//                 }
//             }
//         }
//     }
// }
