use crate::digraph::traits::Vertices;

use super::{Arity, HeightOne, Operation, Partition, Precolor};

/// s(a,r,e,a) = s(r,a,r,e)
#[derive(Clone, Copy, Debug)]
pub struct Siggers;

impl Precolor for Siggers {}

impl Operation for Siggers {
    fn arity(&self) -> Arity {
        4
    }

    fn partition<G>(&self, graph: &G) -> Partition<Vec<G::Vertex>>
    where
        for<'a> G: Vertices<'a>,
    {
        let mut vec = Vec::new();

        for x in graph.vertices() {
            for y in graph.vertices() {
                for z in graph.vertices() {
                    if x != y || y != z {
                        if y == z {
                            vec.push(vec![vec![x, y, z, x], vec![y, x, y, z], vec![x, z, x, y]]);
                        } else if x != z {
                            vec.push(vec![vec![x, y, z, x], vec![y, x, y, z]]);
                        }
                    }
                }
            }
        }
        vec
    }
}

impl HeightOne for Siggers {
    #[allow(clippy::many_single_char_names)]
    fn eq_under<T: PartialEq>(v: &[T], w: &[T]) -> bool {
        fn cmp1<T: PartialEq>(x: &[T], y: &[T]) -> bool {
            let a = x[0] == y[1] && x[0] == x[3];
            let r = x[1] == y[0] && x[1] == y[2];
            let e = x[2] == y[3];
            r && a && e
        }
        fn cmp2<T: PartialEq>(x: &[T], y: &[T]) -> bool {
            if x[0] == x[3] && x[1] == x[2] {
                let a = x[0] == y[0] && x[3] == y[2];
                let r = x[1] == y[1];
                let e = x[2] == y[3];
                return r && a && e;
            }
            false
        }
        debug_assert!(v.len() == 4 && w.len() == 4, "length must be equal to 4!");
        cmp1(v, w) || cmp1(w, v) || cmp2(v, w) || cmp2(w, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_siggers() {
        assert!(Siggers::eq_under(&[0, 1, 2, 0], &[1, 0, 1, 2]));
        assert!(Siggers::eq_under(&[1, 0, 1, 2], &[0, 1, 2, 0]));
    }
}
