use crate::digraph::traits::{Edges, GraphType};

/// A homomorphism from G to H is a mapping h: V(G) → V(H) such that
/// (h(u),h(v)) ∈ E(H) if (u,v) ∈ E(G).
///
/// This property cannot be checked by the compiler.
pub trait Homomorphism<VG, VH>: FnMut(&VG) -> VH {}

/// An endomorphism of a digraph H is a homomorphism from H to H.
pub trait Endomorphism<V>: Homomorphism<V, V> {}

/// A polymorphism of H is a homomorphism from H<sup>k</sup> to H where H
/// is a digraph and k ≥ 1.
pub trait Polymorphism<V>: Homomorphism<Vec<V>, V> {}

pub fn is_homomorphism<'a, G, H, F>(mut f: F, g: &'a G, h: &H) -> bool
where
    G: Edges<'a>,
    H: Edges<'a>,
    F: FnMut(G::Vertex) -> H::Vertex,
{
    for (u, v) in g.edges() {
        if !h.has_edge(f(u), f(v)) {
            return false;
        }
    }
    true
}

pub fn is_endomorphism<'a, H: 'a, F>(f: F, h: &'a H) -> bool
where
    H: Edges<'a>,
    F: FnMut(H::Vertex) -> H::Vertex,
{
    is_homomorphism(f, h, h)
}

// pub fn is_polymorphism<'a, H, F>(f: F, h: &H, k: usize) -> bool
// where
//     H: Edges<'a>,
//     H::EdgeIter: Clone,
//     F: FnMut(Vec<H::Vertex>) -> H::Vertex,
// {
//     is_homomorphism(f, &h.edges().power(k), h)
// }

impl<V, I> GraphType for I
where
    V: Copy + Eq,
    I: Iterator<Item = (V, V)>,
{
    type Vertex = V;
}

impl<'a, V, I> Edges<'a> for I
where
    V: Copy + Eq,
    I: Iterator<Item = (V, V)> + Clone,
{
    type EdgeIter = Self;

    fn edges(&'a self) -> Self::EdgeIter {
        self.clone()
    }

    fn edge_count(&self) -> usize {
        todo!()
    }

    fn has_edge(&self, _u: Self::Vertex, _v: Self::Vertex) -> bool {
        todo!()
    }
}

impl<V: Clone, I> IterAlgebra<V> for I where I: Iterator<Item = (V, V)> {}

#[doc(hidden)]
pub trait IterAlgebra<V: Clone>: Iterator<Item = (V, V)> {
    fn power(self, n: usize) -> Power<V>
    where
        Self: Sized + Clone,
    {
        let mut edges = vec![(vec![], vec![])];

        for _ in 0..n {
            let mut t_edges = Vec::<(Vec<V>, Vec<V>)>::new();
            edges.into_iter().for_each(|(u, v)| {
                for (e0, e1) in self.clone() {
                    let mut u_t = u.clone(); // TODO shorten with extend_one when stable
                    let mut v_t = v.clone();
                    u_t.push(e0);
                    v_t.push(e1);
                    t_edges.push((u_t, v_t));
                }
            });
            edges = t_edges;
        }
        Power(edges.into_iter())
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct Power<V>(std::vec::IntoIter<(Vec<V>, Vec<V>)>);

impl<V> Iterator for Power<V> {
    type Item = (Vec<V>, Vec<V>);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
