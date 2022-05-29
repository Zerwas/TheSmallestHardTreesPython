use std::fmt;
use std::iter::{Copied, Iterator};
use std::slice::Iter;

use bit_vec::BitVec;
use num_iter::{range, Range};
use num_traits::{PrimInt, Unsigned};

use super::classes::Buildable;
use super::traits::{Digraph, Edges, GraphType, Vertices};

#[derive(Clone, Debug)]
pub struct AdjMatrix<V = usize> {
    num_vertices: usize,
    adj: BitVec,
    edges: Vec<(V, V)>,
}

impl AdjMatrix {
    pub fn build_from<'a, G>(g: &'a G) -> AdjMatrix<G::Vertex>
    where
        G: Digraph<'a>,
        G::Vertex: PrimInt + Unsigned,
    {
        let mut m = AdjMatrix::with_capacities(g.vertex_count(), g.edge_count());
        for _ in 0..g.vertex_count() {
            m.add_vertex();
        }
        for (u, v) in g.edges() {
            m.add_edge(u, v);
        }
        m
    }
}

impl<V: PrimInt + Unsigned> Buildable for AdjMatrix<V> {
    type Vertex = V;

    fn with_capacities(nvertices: usize, nedges: usize) -> Self {
        AdjMatrix {
            num_vertices: 0,
            adj: BitVec::with_capacity(nvertices * nvertices),
            edges: Vec::with_capacity(nedges),
        }
    }

    fn add_vertex(&mut self) -> V {
        let id = V::from(self.num_vertices).unwrap();
        self.adj.extend(vec![false; 2 * self.num_vertices + 1]);
        self.num_vertices += 1;
        id
    }

    fn add_edge(&mut self, u: V, v: V) {
        let u_id = u.to_usize().unwrap();
        let v_id = v.to_usize().unwrap();
        if u_id >= self.num_vertices {
            for _ in self.num_vertices..=u_id {
                self.add_vertex();
            }
        }
        if v_id >= self.num_vertices {
            for _ in self.num_vertices..=v_id {
                self.add_vertex();
            }
        }
        let edge_id = u_id * self.vertex_count() + v_id;
        self.adj.set(edge_id, true);
        self.edges.push((u, v));
    }

    fn shrink_to_fit(&mut self) {
        self.adj.shrink_to_fit();
        self.edges.shrink_to_fit();
    }
}

impl<'a, V> GraphType for AdjMatrix<V>
where
    V: 'a + PrimInt + Unsigned,
{
    type Vertex = V;
}

#[derive(Clone)]
pub struct VertexIt<V>(Range<V>);

impl<V> Iterator for VertexIt<V>
where
    V: PrimInt + Unsigned,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a, V> Vertices<'a> for AdjMatrix<V>
where
    V: 'a + PrimInt + Unsigned,
{
    type VertexIter = VertexIt<V>;

    fn vertices(&self) -> Self::VertexIter {
        VertexIt(range(V::zero(), V::from(self.vertex_count()).unwrap()))
    }

    fn vertex_count(&self) -> usize {
        self.num_vertices
    }

    fn has_vertex(&self, v: Self::Vertex) -> bool {
        v < V::from(self.num_vertices).unwrap()
    }
}

#[derive(Clone)]
pub struct EdgeIt<'a, V: Copy>(Copied<Iter<'a, (V, V)>>);

impl<V: Copy> Iterator for EdgeIt<'_, V> {
    type Item = (V, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a, V> Edges<'a> for AdjMatrix<V>
where
    V: 'a + PrimInt + Unsigned,
{
    type EdgeIter = EdgeIt<'a, V>;

    fn edges(&'a self) -> Self::EdgeIter {
        let t = self.edges.iter().copied();
        EdgeIt(t)
    }

    fn edge_count(&self) -> usize {
        self.edges.len()
    }

    fn has_edge(&self, u: Self::Vertex, v: Self::Vertex) -> bool {
        self.adj[u.to_usize().unwrap() * self.vertex_count() + v.to_usize().unwrap()]
    }
}

impl Digraph<'_> for AdjMatrix {}

impl<V> FromIterator<(V, V)> for AdjMatrix<V> {
    fn from_iter<T: IntoIterator<Item = (V, V)>>(_iter: T) -> AdjMatrix<V> {
        todo!()
    }
}

impl fmt::Display for AdjMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("[");
        for (u, v) in self.edges() {
            s.push_str(&format!("({},{})", u, v));
        }
        write!(f, "")
    }
}
