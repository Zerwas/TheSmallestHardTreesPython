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

pub fn is_homomorphism<VG, VH, G, H, F>(mut f: F, g: G, h: H) -> bool
where
    G: Edges<VG>,
    H: HasEdge<VH>,
    F: FnMut(&VG) -> VH,
{
    for (u, v) in g.edge_iter() {
        if !h.has_edge(&f(&u), &f(&v)) {
            return false;
        }
    }
    true
}

// pub fn is_endomorphism<VH, H, F>(f: F, h: H) -> bool
// where
//     H: Edges<VH> + HasEdge<VH>,
//     F: FnMut(&VH) -> VH,
// {
//     is_homomorphism(f, h, h)    // TODO fix Edges<VH> trait, h is moved
// }

pub fn is_polymorphism<VH, H, F>(f: F, h: H, k: usize) -> bool
where
    VH: Clone,
    H: Edges<VH> + HasEdge<VH> + Clone,
    F: FnMut(&Vec<VH>) -> VH,
{
    is_homomorphism(f, h.edge_iter().power(k), h)
}

pub trait HasEdge<V> {
    fn has_edge(&self, v: &V, w: &V) -> bool;
}

pub trait Vertices<V> {
    type VerticesIter: Iterator<Item = V>;

    fn vertex_iter(&self) -> Self::VerticesIter;
}

impl<V: Clone, I> IterAlgebra<V> for I where I: Iterator<Item = (V, V)> {}

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

#[derive(Clone, Debug)]
pub struct Power<V>(std::vec::IntoIter<(Vec<V>, Vec<V>)>);

// impl<V> IntoEdges<Vec<V>> for Power<V> {
//     type EdgesIter = Self;

//     fn into_edges(self) -> Self::EdgesIter {
//         self
//     }
// }

impl<V> Iterator for Power<V> {
    type Item = (Vec<V>, Vec<V>);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<V: Clone> Edges<Vec<V>> for Power<V> {
    type EdgesIter = Self;

    fn edge_iter(&self) -> Self::EdgesIter {
        self.clone()
    }
}

pub trait Edges<V> {
    type EdgesIter: Iterator<Item = (V, V)> + Clone;

    fn edge_iter(&self) -> Self::EdgesIter;
}

// pub trait IntoEdges<V> {
//     type EdgesIter: Iterator<Item = (V, V)>;

//     fn into_edges(self) -> Self::EdgesIter;
// }
