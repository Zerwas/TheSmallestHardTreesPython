use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use bit_vec::BitVec;

#[derive(Debug, Clone)]
struct Vertex<T> {
    value: T,
    out_edges: BitVec,
}

impl<T> Vertex<T> {
    fn new(value: T, vertex_count: usize) -> Vertex<T> {
        Vertex {
            value,
            out_edges: BitVec::from_elem(vertex_count, false),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AdjMatrix<T> {
    vertices: Vec<Vertex<T>>,
    indices: HashMap<T, usize>,
    edges: HashSet<(usize, usize)>,
}

impl<T: Hash + Eq + Clone> AdjMatrix<T> {
    pub fn new() -> AdjMatrix<T> {
        AdjMatrix {
            vertices: Vec::new(),
            indices: HashMap::new(),
            edges: HashSet::new(),
        }
    }

    pub fn from_vertices(vs: Vec<T>) -> AdjMatrix<T> {
        let vertex_count = vs.len();
        let mut vertices = Vec::with_capacity(vertex_count);

        for v in &vs {
            vertices.push(Vertex {
                value: v.clone(),
                out_edges: BitVec::from_elem(vertex_count, false),
            });
        }
        let indices = vs.into_iter().enumerate().map(|(u, v)| (v, u)).collect();

        AdjMatrix {
            vertices,
            indices,
            edges: HashSet::new(),
        }
    }

    /// Create a new `AdjMatrix` from an iterable of edges.
    pub fn from_edges<I>(edges: I) -> AdjMatrix<T>
    where
        I: IntoIterator<Item = (T, T)>,
    {
        Self::from_iter(edges)
    }

    /// Adds a new vertex to the graph.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMatrix;
    ///
    /// let mut graph = AdjMatrix::new();
    ///
    /// graph.add_vertex(1);
    ///
    /// assert_eq!(graph.vertex_count(), 1);
    /// ```
    pub fn add_vertex(&mut self, a: T) -> usize {
        let id = self.vertices.len();
        for v in &mut self.vertices {
            v.out_edges.push(false);
        }
        self.vertices.push(Vertex::new(a.clone(), id + 1));
        self.indices.insert(a, id);
        id
    }

    /// Attempts to place a new edge in the graph.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMatrix;
    ///
    /// let mut graph = AdjMatrix::new();
    ///
    /// let v1 = graph.add_vertex(1);
    /// let v2 = graph.add_vertex(2);
    ///
    /// // Adding an edge is idempotent
    /// graph.add_edge(v1, v2);
    /// graph.add_edge(v1, v2);
    ///
    /// assert_eq!(graph.edges_count(), 1);
    /// ```
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.vertices[u].out_edges.set(v, true);
        self.edges.insert((u, v));
    }
}

impl<T> AdjMatrix<T> {
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMatrix;
    ///
    /// let mut graph = AdjMatrix::new();
    ///
    /// let v1 = graph.add_vertex(1);
    /// let v2 = graph.add_vertex(3);
    ///
    /// assert_eq!(graph.value(v1), &1);
    /// assert_eq!(graph.value(v2), &3);
    /// ```
    pub fn value(&self, vertex_id: usize) -> &T {
        &self.vertices[vertex_id].value
    }

    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMatrix;
    ///
    /// let mut graph = AdjMatrix::new();
    ///
    /// let v1 = graph.add_vertex(1);
    /// let v2 = graph.add_vertex(3);
    ///
    /// assert_eq!(graph.values().len(), 2);
    /// ```
    pub fn values(&self) -> Vec<&T> {
        self.vertices.iter().map(|v| &v.value).collect()
    }

    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMatrix;
    ///
    /// let mut graph = AdjMatrix::new();
    ///
    /// let v1 = graph.add_vertex(1);
    /// let v2 = graph.add_vertex(3);
    ///
    /// assert_eq!(graph.vertices(), vec![0, 1]);
    /// ```
    pub fn vertices(&self) -> Vec<usize> {
        (0..self.vertices.len()).collect()
    }

    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMatrix;
    ///
    /// let mut graph = AdjMatrix::new();
    ///
    /// let v1 = graph.add_vertex(1);
    /// let v2 = graph.add_vertex(3);
    ///
    /// assert_eq!(graph.vertex_count(), 2);
    /// ```
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }
}

impl<T: Hash + Eq> AdjMatrix<T> {
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMatrix;
    ///
    /// let mut graph = AdjMatrix::new();
    ///
    /// let v1 = graph.add_vertex(1);
    /// let v2 = graph.add_vertex(3);
    ///
    /// assert_eq!(graph.index_of(&1), Some(v1));
    /// assert_eq!(graph.index_of(&3), Some(v2));
    /// assert_eq!(graph.index_of(&4), None);
    /// ```
    pub fn index_of(&self, v: &T) -> Option<usize> {
        self.indices.get(v).copied()
    }
}

impl<T> AdjMatrix<T> {
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMatrix;
    ///
    /// let mut graph = AdjMatrix::new();
    ///
    /// let v1 = graph.add_vertex(1);
    /// let v2 = graph.add_vertex(3);
    /// let v3 = graph.add_vertex(5);
    ///
    /// graph.add_edge(v1, v2);
    /// graph.add_edge(v3, v2);
    /// graph.add_edge(v2, v3);
    ///
    /// assert_eq!(graph.edges().count(), 3);
    /// ```
    pub fn edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.edges.iter().copied()
    }

    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMatrix;
    ///
    /// let mut graph = AdjMatrix::new();
    ///
    /// let v1 = graph.add_vertex(1);
    /// let v2 = graph.add_vertex(3);
    /// let v3 = graph.add_vertex(5);
    ///
    /// graph.add_edge(v1, v2);
    /// graph.add_edge(v3, v2);
    /// graph.add_edge(v2, v3);
    ///
    /// assert_eq!(graph.edges_count(), 3);
    /// ```
    pub fn edges_count(&self) -> usize {
        self.edges().count()
    }

    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMatrix;
    ///
    /// let mut graph = AdjMatrix::new();
    ///
    /// let v1 = graph.add_vertex(1);
    /// let v2 = graph.add_vertex(3);
    /// let v3 = graph.add_vertex(5);
    ///
    /// graph.add_edge(v1, v2);
    /// graph.add_edge(v3, v2);
    /// graph.add_edge(v2, v3);
    ///
    /// assert!(graph.has_edge(v1, v2));
    /// assert!(graph.has_edge(v2, v3));
    /// assert!(graph.has_edge(v3, v2));
    /// ```
    pub fn has_edge(&self, v: usize, w: usize) -> bool {
        self.vertices[v].out_edges[w]
    }
}

impl<V: Hash + Eq + Clone> FromIterator<(V, V)> for AdjMatrix<V> {
    fn from_iter<T: IntoIterator<Item = (V, V)>>(iter: T) -> Self {
        let mut result = AdjMatrix::new();

        for (u, v) in iter {
            let ui = result.index_of(&u).unwrap_or(result.add_vertex(u));
            let vi = result.index_of(&v).unwrap_or(result.add_vertex(v));
            result.add_edge(ui, vi);
        }

        result
    }
}
