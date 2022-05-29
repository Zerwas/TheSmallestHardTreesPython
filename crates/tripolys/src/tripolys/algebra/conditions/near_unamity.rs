use itertools::Itertools;

use std::collections::HashMap;
use std::hash::Hash;

use crate::digraph::traits::Vertices;

use super::{Arity, Operation, Partition, Precolor};

impl Operation for Majority {
    fn arity(&self) -> Arity {
        3
    }

    fn partition<G>(&self, vertices: &G) -> Partition<Vec<G::Vertex>>
    where
        for<'a> G: Vertices<'a>,
    {
        nu(3).partition(vertices)
    }
}

impl Operation for Wnu {
    fn arity(&self) -> Arity {
        self.arity
    }

    fn partition<G>(&self, vertices: &G) -> Partition<Vec<G::Vertex>>
    where
        for<'a> G: Vertices<'a>,
    {
        nu_partition(self.arity(), vertices, true)
    }
}

impl Operation for Nu {
    fn arity(&self) -> Arity {
        self.arity
    }

    fn partition<G>(&self, vertices: &G) -> Partition<Vec<G::Vertex>>
    where
        for<'a> G: Vertices<'a>,
    {
        nu_partition(self.arity(), vertices, false)
    }
}

/// m(x,x,y) = m(x,y,x) = m(y,x,x) = m(x,x,x) = x
#[derive(Clone, Copy, Debug)]
pub struct Majority {}

pub fn majority() -> Majority {
    Majority {}
}

impl Precolor for Majority {
    fn precolor<V: PartialEq + Copy>(&self, (_, v): &(usize, Vec<V>)) -> Option<V> {
        if v.iter().all_equal() {
            Some(v[0])
        } else {
            None
        }
    }
}

/// f(x,...,x,y) = f(x,...,x,y,x) = ... = f(y,x,...,x) = x
#[derive(Clone, Copy, Debug)]
pub struct Nu {
    arity: usize,
}

pub fn nu(arity: usize) -> Nu {
    Nu { arity }
}

impl Precolor for Nu {
    fn precolor<V: Copy + Eq + Hash>(&self, (_, v): &(usize, Vec<V>)) -> Option<V> {
        if let ElemCount::Once(x1, _) = elem_count(v) {
            Some(x1)
        } else {
            None
        }
    }
}

/// f(x,...,x,y) = f(x,...,x,y,x) = ... = f(y,x,...,x)
#[derive(Clone, Copy, Debug)]
pub struct Wnu {
    arity: usize,
}

pub fn wnu(arity: usize) -> Wnu {
    Wnu { arity }
}

impl Precolor for Wnu {}

fn nu_partition<G>(arity: usize, g: &G, weak: bool) -> Partition<Vec<G::Vertex>>
where
    for<'a> G: Vertices<'a>,
{
    let mut partition = Vec::new();

    for v in g.vertices() {
        let mut vec = Vec::new();

        for w in g.vertices() {
            if v == w {
                continue;
            }
            let mut eq_class = Vec::new();

            for k in 0..arity {
                let mut tuple = vec![v; arity];
                tuple[k] = w;
                eq_class.push(tuple);
            }
            if weak {
                partition.push(eq_class);
            } else {
                vec.push(eq_class);
            }
        }
        if !weak {
            partition.push(vec.into_iter().flatten().collect_vec());
        }
    }

    partition
}

// impl HeightOne for Nu {
//     fn eq_under<V>(t1: &[V], t2: &[V]) -> bool {
//         assert!(t1.len() >= 2 && t2.len() >= 2, "length must be at least 2!");
//         match (elem_count(t1), elem_count(t2)) {
//             (ElemCount::Once(x1, _), ElemCount::Once(x2, _)) => x1 == x2,
//             _ => false,
//         }
//     }
// }

// impl HeightOne for Wnu {
//     fn eq_under<V: Copy + Hash + Eq>(t1: &[V], t2: &[V]) -> bool {
//         assert!(t1.len() >= 2 && t2.len() >= 2, "length must be at least 2!");
//         match (elem_count(t1), elem_count(t2)) {
//             (ElemCount::Once(x1, y1), ElemCount::Once(x2, y2)) => x1 == x2 && y1 == y2,
//             _ => false,
//         }
//     }
// }

// Result of pre-processing for WNU/quasi-majority check
enum ElemCount<T: Eq + Clone + Hash> {
    // (x, x, x, y) -> (x, y)
    Once(T, T),
    // (x, x, x) -> (x)
    AllEqual(T),
    // (x, y, z, x, x) -> ()
    None,
}

fn elem_count<T: Eq + Clone + Hash>(x: &[T]) -> ElemCount<T> {
    // (elem, frequency of elem)
    let elem_freq = x.iter().fold(HashMap::<T, usize>::new(), |mut m, y| {
        *m.entry(y.clone()).or_default() += 1;
        m
    });

    match elem_freq.len() {
        1 => ElemCount::AllEqual(elem_freq.keys().next().cloned().unwrap()),
        2 => {
            let mut it = elem_freq.into_iter();
            let (e0, f0) = it.next().unwrap();
            let (e1, f1) = it.next().unwrap();
            if f0 == 1 {
                ElemCount::Once(e1, e0)
            } else if f1 == 1 {
                ElemCount::Once(e0, e1)
            } else {
                ElemCount::None
            }
        }
        _ => ElemCount::None,
    }
}
