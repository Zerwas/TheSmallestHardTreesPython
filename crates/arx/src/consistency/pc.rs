use std::collections::{HashMap, HashSet};

use crate::domains::Domains;
use crate::problem::*;
use crate::set::Set;

// /// Implementation of the PC-2 algorithm due to Mackworth 1977.
// ///
// /// Returns false, if an empty list is derived for some vertex v, true otherwise.
// pub fn pc_2(domains: &mut Domains, constraints: &impl Constraints) -> bool {
//     let mut lists = Vec::<Vec<HashSet<(Value, Value)>>>::new();
//     let mut pending_list = Set::<(Var, Var, Var)>::new();
//     let mut set = HashSet::<(Value, Value)>::new();

//     for u in g1.vertices() {
//         for v in g1.vertices() {
//             set.insert((u.clone(), v.clone()));
//         }
//     }

//     for u in domains.vars() {
//         for v in domains.vars() {
//             if u == v {
//                 let mut s = Set::<(Value, Value)>::new();
//                 for fuck in g1.vertices() {
//                     s.insert((fuck.clone(), fuck.clone()));
//                 }
//                 lists.insert((u.clone(), v.clone()), s);
//             } else if g0.has_edge(u, v) {
//                 let s = g1.edges().collect::<Set<_>>();
//                 lists.insert((u.clone(), v.clone()), s);
//             } else {
//                 lists.insert((u.clone(), v.clone()), set.clone());
//             }
//             for w in g0.vertices() {
//                 pending_list.insert((u.clone(), w.clone(), v.clone()));
//             }
//         }
//     }

//     while let Some((x, y, z)) = pending_list.pop() {
//         if path_reduce(x, y, z, &mut lists) {
//             // list of x,y changed, was the empty list derived?
//             if lists.get(&(x.clone(), y.clone())).unwrap().is_empty() {
//                 return false;
//             }
//             for u in g0.vertices() {
//                 if *u != x && *u != y {
//                     pending_list.insert((u.clone(), x.clone(), y.clone()));
//                     pending_list.insert((u.clone(), y.clone(), x.clone()));
//                 }
//             }
//         }
//     }
//     true
// }

// // Implementation of the path-reduce operation from pc2.
// // Returns true, if the list of x,y was reduced, false otherwise.
// fn path_reduce(x: Var, y: Var, z: Var, lists: &mut HashMap<(X, X), Set<(D, D)>>) -> bool {
//     for (a, b) in lists.get(&(x.clone(), y.clone())).unwrap().clone() {
//         'middle: for (u, v) in lists.get(&(x.clone(), z.clone())).unwrap() {
//             if a == u {
//                 for (c, d) in lists.get(&(y.clone(), z.clone())).unwrap() {
//                     if c == b && d == v {
//                         break 'middle;
//                     }
//                 }
//             }
//         }
//     }
//     false
// }

// pub trait PathConsistency {}
