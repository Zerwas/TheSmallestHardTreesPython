//! A graph datastructure using an adjacency list representation.
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use itertools::Itertools;

use crate::algebra::{Edges, Vertices};

use super::AdjMatrix;

pub trait VertexId: Eq + Clone + Hash + Debug + Send + Sync {}

impl<V> VertexId for V where V: Eq + Clone + Hash + Debug + Send + Sync {}

/// Graph<V> is a directed graph datastructure using an adjacency list
/// representation.
///
/// For each vertex the `HashMap` contains an ordered pair, the adjacency
/// lists, where the first entry and second entry contain all successors and
/// predecessors, respectively.
#[derive(Debug, Clone, Default)]
pub struct AdjMap<V: VertexId> {
    // Vertex -> (Out-Edges, In-Edges)
    lists: HashMap<V, (HashSet<V>, HashSet<V>)>,
    edges: HashSet<(V, V)>,
}

impl<T: VertexId> AdjMap<T> {
    /// Creates an empty `Graph`.
    pub fn new() -> AdjMap<T> {
        AdjMap {
            lists: HashMap::new(),
            edges: HashSet::new(),
        }
    }

    // TODO
    pub fn with_capacity(capacity: usize) -> AdjMap<T> {
        AdjMap {
            lists: HashMap::with_capacity(capacity),
            edges: HashSet::with_capacity(capacity),
        }
    }

    /// Adds a new vertex to the graph.
    ///
    /// If the graph did not have this vertex present, `true` is returned.
    ///
    /// If the graph did have this vertex present, `false` is returned.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    ///
    /// let mut graph = AdjMap::<u32>::new();
    ///
    /// assert!(graph.add_vertex(1));
    /// assert!(!graph.add_vertex(1));
    ///
    /// assert_eq!(graph.vertex_count(), 1);
    /// ```
    pub fn add_vertex(&mut self, v: T) -> bool {
        if self.has_vertex(&v) {
            false
        } else {
            self.lists.insert(v, (HashSet::new(), HashSet::new()));
            true
        }
    }

    /// Removes a vertex from the graph.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    ///
    /// let mut graph = AdjMap::<u32>::new();
    ///
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    ///
    /// // The remove operation is idempotent
    /// graph.remove_vertex(&2);
    /// graph.remove_vertex(&2);
    /// graph.remove_vertex(&2);
    ///
    /// assert_eq!(graph.vertex_count(), 2);
    /// ```
    pub fn remove_vertex(&mut self, v: &T) {
        if let Some((out_edges, in_edges)) = self.lists.remove(v) {
            // remove vertex from in-edge list of other vertices
            for u in &out_edges {
                self.lists.get_mut(u).unwrap().1.remove(v);
                self.edges.remove(&(v.clone(), u.clone()));
            }

            // remove vertex from out-edge list of other vertices
            for u in &in_edges {
                self.lists.get_mut(u).unwrap().0.remove(v);
                self.edges.remove(&(u.clone(), v.clone()));
            }
        }
    }

    /// Returns `true` if the graph contains the given vertex, false otherwise.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    ///
    /// let mut graph = AdjMap::<u32>::new();
    ///
    /// graph.add_vertex(1);
    ///
    /// assert!(graph.has_vertex(&1));
    /// assert!(!graph.has_vertex(&2));
    /// ```
    pub fn has_vertex(&self, v: &T) -> bool {
        self.lists.contains_key(v)
    }

    /// Returns an iterator over references to all of the vertices in the graph.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    ///
    /// let mut graph = AdjMap::<u32>::new();
    /// let mut vertices = vec![];
    ///
    /// graph.add_vertex(0);
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    ///
    /// // Iterate over vertices
    /// for v in graph.vertices() {
    ///     vertices.push(v);
    /// }
    ///
    /// assert_eq!(vertices.len(), 4);
    /// ```
    pub fn vertices(&self) -> VertexIter<T> {
        VertexIter(Box::new(self.lists.keys()))
    }

    /// Returns the number of vertices that are placed in
    /// the graph.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    ///
    /// let mut graph = AdjMap::<u32>::new();
    ///
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    ///
    /// assert_eq!(graph.vertex_count(), 3);
    /// ```
    pub fn vertex_count(&self) -> usize {
        self.lists.len()
    }

    /// Attempts to place a new edge in the graph.
    ///
    /// If the graph did not have this edge present, `true` is returned.
    ///
    /// If the graph did have this edge present, `false` is returned.
    ///
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    ///
    /// let mut graph = AdjMap::<u32>::new();
    ///
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    ///
    /// // Adding an edge is idempotent
    /// assert!(graph.add_edge(&1, &2));
    /// assert!(!graph.add_edge(&1, &2));
    /// ```
    pub fn add_edge(&mut self, u: &T, v: &T) -> bool {
        if self.has_edge(u, v) {
            false
        } else {
            if let Some((out_edges, _)) = self.lists.get_mut(u) {
                out_edges.insert(v.clone());
            } else {
                panic!("Vertex with id {:?} doesn't exist", u);
            }
            if let Some((_, in_edges)) = self.lists.get_mut(v) {
                in_edges.insert(u.clone());
            } else {
                panic!("Vertex with id {:?} doesn't exist", v);
            }
            self.edges.insert((u.clone(), v.clone()));
            true
        }
    }

    /// Removes an edge from the graph, returning true if the edge was previously
    /// present, false otherwise.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    ///
    /// let mut graph = AdjMap::<u32>::new();
    ///
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    /// graph.add_vertex(4);
    ///
    /// graph.add_edge(&1, &2);
    /// graph.add_edge(&2, &3);
    /// graph.add_edge(&3, &4);
    ///
    /// assert_eq!(graph.edge_count(), 3);
    ///
    /// // The remove edge operation is idempotent
    /// assert!(graph.remove_edge(&2, &3));
    /// assert!(!graph.remove_edge(&2, &3));
    ///
    /// assert_eq!(graph.edge_count(), 2);
    /// ```
    pub fn remove_edge(&mut self, u: &T, v: &T) -> bool {
        if self.has_edge(u, v) {
            self.lists.get_mut(u).unwrap().0.remove(v);
            self.lists.get_mut(v).unwrap().1.remove(u);
            self.edges.remove(&(u.clone(), v.clone()));
            true
        } else {
            false
        }
    }

    /// Returns `true` if the graph contains the given edge, false otherwise.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    ///
    /// let mut graph = AdjMap::<u32>::new();
    ///
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    ///
    /// graph.add_edge(&1, &2);
    ///
    /// assert!(graph.has_edge(&1, &2));
    /// assert!(!graph.has_edge(&2, &3));
    /// ```
    pub fn has_edge(&self, u: &T, v: &T) -> bool {
        self.lists.get(u).unwrap().0.contains(v)
    }

    /// Returns an iterator over all of the edges in the graph.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    ///
    /// let mut graph = AdjMap::<u32>::new();
    /// let mut edges = vec![];
    ///
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    /// graph.add_vertex(4);
    ///
    /// graph.add_edge(&1, &2);
    /// graph.add_edge(&3, &1);
    /// graph.add_edge(&1, &4);
    ///
    /// // Iterate over edges
    /// for v in graph.edges() {
    ///     edges.push(v);
    /// }
    ///
    /// assert_eq!(edges.len(), 3);
    /// ```
    pub fn edges(&self) -> impl Iterator<Item = (T, T)> + '_ {
        self.edges.iter().cloned()
    }

    /// Returns the total number of edges that are listed
    /// in the graph.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    ///
    /// let mut graph = AdjMap::<u32>::new();
    ///
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    /// graph.add_vertex(4);
    ///
    /// graph.add_edge(&1, &2);
    /// graph.add_edge(&2, &3);
    /// graph.add_edge(&3, &4);
    ///
    /// println!("{:?}", graph.edges().collect::<Vec<_>>());
    /// assert_eq!(graph.edge_count(), 3);
    /// ```
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns an iterator over the inbound and outbound neighbors
    /// of the vertex `v`.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    /// use std::collections::HashSet;
    ///
    /// let mut graph = AdjMap::<u32>::new();
    ///
    /// graph.add_vertex(0);
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    ///
    /// graph.add_edge(&0, &1);
    /// graph.add_edge(&2, &0);
    /// graph.add_edge(&0, &3);
    ///
    /// let neighbors = graph.neighbors(&0).collect::<HashSet<_>>();
    ///
    /// assert!(neighbors.contains(&1));
    /// assert!(neighbors.contains(&2));
    /// assert!(neighbors.contains(&3));
    /// ```
    pub fn neighbors(&self, v: &T) -> VertexIter<T> {
        let (o, i) = self.lists.get(v).unwrap();
        VertexIter(Box::new(o.iter().chain(i.iter())))
    }

    /// Returns the total count of neighboring vertices of the vertex `x`.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    ///
    /// let mut graph = AdjMap::<u32>::new();
    ///
    /// graph.add_vertex(0);
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    ///
    /// graph.add_edge(&0, &1);
    /// graph.add_edge(&2, &0);
    /// graph.add_edge(&0, &3);
    ///
    /// assert_eq!(graph.neighbors_count(&0), 3);
    /// ```
    pub fn neighbors_count(&self, v: &T) -> usize {
        let (o, i) = self.lists.get(v).unwrap();
        o.len() + i.len()
    }

    /// Returns an iterator over the outbound neighbors of the vertex `v`.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    /// use itertools::Itertools;
    ///
    /// let mut graph = AdjMap::<u32>::new();
    /// let mut neighbors = vec![];
    ///
    /// graph.add_vertex(0);
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    ///
    /// graph.add_edge(&0, &1);
    /// graph.add_edge(&2, &0);
    /// graph.add_edge(&3, &0);
    ///
    /// // Iterate over neighbors
    /// for v in graph.out_neighbors(&0) {
    ///     neighbors.push(v);
    /// }
    ///
    /// assert_eq!(neighbors.len(), 1);
    /// assert_eq!(neighbors[0], &1);
    /// ```
    pub fn out_neighbors(&self, v: &T) -> VertexIter<T> {
        let (o, _) = self.lists.get(v).unwrap();
        VertexIter(Box::new(o.iter()))
    }

    /// Returns the total count of outbound neighboring vertices
    /// of the vertex `v`.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    ///
    /// let mut graph = AdjMap::<u32>::new();
    ///
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    /// graph.add_vertex(4);
    /// graph.add_vertex(5);
    ///
    /// graph.add_edge(&1, &2);
    /// graph.add_edge(&3, &1);
    /// graph.add_edge(&1, &4);
    /// graph.add_edge(&2, &5);
    /// graph.add_edge(&2, &3);
    ///
    /// assert_eq!(graph.out_neighbors_count(&1), 2);
    /// assert_eq!(graph.out_neighbors_count(&2), 2);
    /// ```
    pub fn out_neighbors_count(&self, v: &T) -> usize {
        let (o, _) = self.lists.get(v).unwrap();
        o.len()
    }

    /// Returns an iterator over the inbound neighbors
    /// of the vertex `v`.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    ///
    /// let mut graph = AdjMap::<u32>::new();
    /// let mut neighbors = vec![];
    ///
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    /// graph.add_vertex(4);
    ///
    /// graph.add_edge(&1, &2);
    /// graph.add_edge(&3, &1);
    /// graph.add_edge(&1, &4);
    ///
    /// // Iterate over neighbors
    /// for v in graph.in_neighbors(&1) {
    ///     neighbors.push(v);
    /// }
    ///
    /// assert_eq!(neighbors.len(), 1);
    /// assert_eq!(neighbors[0], &3);
    /// ```
    pub fn in_neighbors(&self, v: &T) -> VertexIter<T> {
        let (_, i) = self.lists.get(v).unwrap();
        VertexIter(Box::new(i.iter()))
    }

    /// Returns the total count of inbound edges of the vertex `x`.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    ///
    /// let mut graph = AdjMap::<u32>::new();
    ///
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    /// graph.add_vertex(4);
    ///
    /// graph.add_edge(&1, &2);
    /// graph.add_edge(&3, &1);
    /// graph.add_edge(&1, &4);
    ///
    /// assert_eq!(graph.in_neighbors_count(&1), 1);
    /// ```
    pub fn in_neighbors_count(&self, v: &T) -> usize {
        let (_, i) = self.lists.get(v).unwrap();
        i.len()
    }

    /// Performs a map over all of the vertices of the graph,
    /// applying the given transformation function to each one.
    ///
    /// Returns a new graph with the same edges but with transformed
    /// vertices.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    ///
    /// let mut graph = AdjMap::<u32>::new();
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    ///
    /// graph.add_edge(&1, &2);
    ///
    /// // Map each vertex
    /// let mapped = graph.map(|v| v + 2);
    ///
    /// assert!(graph.has_edge(&1, &2));
    /// assert!(mapped.has_edge(&3, &4));
    /// ```
    pub fn map<U, F>(&self, f: F) -> AdjMap<U>
    where
        U: VertexId,
        F: Fn(&T) -> U,
    {
        AdjMap::from_edges(self.edges().map(|(u, v)| (f(&u), f(&v))))
    }

    /// Contracts the vertex `y` with the vertex `x`. The resulting vertex has
    /// id `x`.
    ///
    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    ///
    /// let mut graph = AdjMap::<u32>::new();
    ///
    /// graph.add_vertex(1);
    /// graph.add_vertex(2);
    /// graph.add_vertex(3);
    /// graph.add_vertex(4);
    ///
    /// graph.add_edge(&1, &2);
    /// graph.add_edge(&2, &3);
    /// graph.add_edge(&1, &4);
    ///
    /// graph.contract_vertices(&2, &4);
    /// graph.contract_vertices(&1, &3);
    ///
    /// // assert_eq!(graph.vertex_count(), 2);
    /// assert_eq!(graph.edge_count(), 2);
    /// ```
    pub fn contract_vertices(&mut self, u: &T, v: &T) -> bool {
        if u == v {
            return true;
        }
        if !self.lists.contains_key(u) || !self.lists.contains_key(v) {
            return false;
        }
        let (o, i) = self.lists.remove(v).unwrap();
        for w in o {
            self.add_edge(u, &w);
            self.lists.get_mut(&w).unwrap().1.remove(v);
            self.edges.remove(&(v.clone(), w.clone()));
        }
        for w in i {
            self.add_edge(&w, u);
            self.lists.get_mut(&w).unwrap().0.remove(v);
            self.edges.remove(&(w.clone(), v.clone()));
        }
        true
    }

    /// TODO Contracts each two vertices specified by the predicate.
    ///
    /// In other words, contract all vertices `v`, `w` such that `f(&v, &w)` returns `true`.
    ///
    /// **NOTE:** This method has running time of O(n^2).  A better running time
    /// can be achieved by generating sets of vertices that must be contracted
    /// and then use the [`Graph::contract_vertices`] method.
    /// [`Graph::contract_vertices`]: ./`struct.Graph.html#method.contract_vertices`
    pub fn contract_if<F>(&mut self, mut f: F)
    where
        F: FnMut(&T, &T) -> bool,
    {
        let vertices = self.vertices().cloned().collect_vec();
        let mut removed = HashSet::<T>::new();

        for (i, v) in vertices.iter().enumerate() {
            if removed.contains(v) {
                continue;
            }
            for j in i + 1..vertices.len() {
                let w = vertices.get(j).unwrap();
                if removed.contains(w) {
                    continue;
                }
                if f(v, w) {
                    self.contract_vertices(v, w);
                    removed.insert(w.clone());
                }
            }
        }
    }

    /// TODO Performs the union of G and H, which is the graph with
    /// vertex set V(G) ∪ V(H).
    ///
    /// **NOTE:** Be aware whether the two vertex sets V(G) and V(H) are
    /// disjoint or not.
    pub fn union(&self, other: &AdjMap<T>) -> AdjMap<T> {
        let mut lists = self.lists.clone();
        lists.extend(other.lists.clone());
        let edges = self.edges().chain(other.edges()).collect();

        AdjMap { lists, edges }
    }

    /// Returns the (weakly) connected components of the graph.
    pub fn connected_components(&self) -> impl Iterator<Item = AdjMap<T>> + '_ {
        self.connected_components_starts(self.vertices().cloned())
    }

    /// Extract connected components from the graph.
    ///
    /// - `starts` is a collection of vertices to be considered as start points.
    ///
    /// Returns a list of disjoint connected components.
    pub fn connected_components_starts<I>(&self, starts: I) -> impl Iterator<Item = AdjMap<T>>
    where
        I: IntoIterator<Item = T>,
    {
        let mut visited = HashSet::<T>::new();
        let mut components = Vec::new();

        for v in starts {
            if !visited.contains(&v) {
                components.push(self.step(v, &mut visited));
            }
        }
        components.into_iter()
    }

    fn step(&self, v: T, visited: &mut HashSet<T>) -> AdjMap<T> {
        let mut component = AdjMap::new();
        let mut stack = vec![v.clone()];
        component.add_vertex(v);

        while let Some(v) = stack.pop() {
            let (out_edges, in_edges) = self.lists.get(&v).unwrap();

            for u in out_edges {
                if !visited.contains(u) {
                    component.add_vertex(u.clone());
                    component.add_edge(&v, u);
                    stack.push(u.clone());
                }
            }
            for u in in_edges {
                if !visited.contains(u) {
                    component.add_vertex(u.clone());
                    component.add_edge(u, &v);
                    stack.push(u.clone());
                }
            }
            visited.insert(v);
        }
        component
    }

    /// TODO
    pub fn from_edges(edges: impl IntoIterator<Item = (T, T)>) -> AdjMap<T> {
        AdjMap::from_iter(edges)
    }

    /// ## Example
    /// ```rust
    /// use tripolys::digraph::AdjMap;
    /// use tripolys::digraph::AdjMatrix;
    ///
    /// let mut map = AdjMap::<u32>::new();
    ///
    /// map.add_vertex(1);
    /// map.add_vertex(3);
    /// map.add_vertex(5);
    ///
    /// map.add_edge(&1, &3);
    /// map.add_edge(&3, &5);
    /// map.add_edge(&5, &3);
    ///
    /// let matrix = map.to_matrix();
    /// let edges = matrix.edges();
    ///
    /// assert!(matrix.has_edge(matrix.index_of(&1).unwrap() , matrix.index_of(&3).unwrap()));
    /// assert!(matrix.has_edge(matrix.index_of(&3).unwrap() , matrix.index_of(&5).unwrap()));
    /// assert!(matrix.has_edge(matrix.index_of(&5).unwrap() , matrix.index_of(&3).unwrap()));
    /// ```
    pub fn to_matrix(&self) -> AdjMatrix<T> {
        let mut mat = AdjMatrix::from_vertices(self.vertices().cloned().collect_vec());

        for (u, v) in self.edges() {
            mat.add_edge(mat.index_of(&u).unwrap(), mat.index_of(&v).unwrap());
        }
        mat
    }
}

pub struct VertexIter<'a, T>(pub(crate) Box<dyn 'a + Iterator<Item = &'a T>>);

impl<'a, T> Iterator for VertexIter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

// TODO criminally hacked
impl<V: VertexId> Vertices<V> for AdjMap<V> {
    type VerticesIter = std::vec::IntoIter<V>;

    fn vertex_iter(&self) -> Self::VerticesIter {
        self.vertices().cloned().collect_vec().into_iter()
    }
}

// TODO criminally hacked
impl<V: VertexId> Edges<V> for AdjMap<V> {
    type EdgesIter = std::vec::IntoIter<(V, V)>;

    fn edge_iter(&self) -> Self::EdgesIter {
        self.edges().collect_vec().into_iter()
    }
}

impl<V: VertexId> FromIterator<(V, V)> for AdjMap<V> {
    fn from_iter<T: IntoIterator<Item = (V, V)>>(iter: T) -> AdjMap<V> {
        let mut graph = AdjMap::<V>::new();
        for (u, v) in iter {
            graph.add_vertex(u.clone());
            graph.add_vertex(v.clone());
            graph.add_edge(&u, &v);
        }
        graph
    }
}

impl<V: VertexId + std::fmt::Display> std::fmt::Display for AdjMap<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("[");

        for (i, (u, v)) in self.edges().enumerate() {
            if i != 0 {
                s.push(',');
            }
            s.push_str(&format!("({:?},{:?})", u, v));
        }
        s.push(']');

        write!(f, "{}", s)
    }
}

impl<V: VertexId> FromIterator<AdjMap<V>> for AdjMap<V> {
    fn from_iter<T: IntoIterator<Item = AdjMap<V>>>(iter: T) -> AdjMap<V> {
        iter.into_iter().fold(AdjMap::new(), |acc, x| acc.union(&x))
    }
}

pub trait ToGraph {
    type V: VertexId;
    /// Converts the given value to a `Graph`.
    fn to_graph(&self) -> AdjMap<Self::V>;
}

impl<T: VertexId> ToGraph for AdjMap<T> {
    type V = T;

    fn to_graph(&self) -> AdjMap<Self::V> {
        self.clone()
    }
}

impl AdjMap<u32> {
    fn complete_graph(n: u32) -> AdjMap<u32> {
        AdjMap::from_edges((0..n).flat_map(|i| [(i, (i + 1) % n), ((i + 1) % n, i)]))
    }

    fn directed_cycle(n: u32) -> AdjMap<u32> {
        AdjMap::from_edges((0..n).map(|i| (i, (i + 1) % n)))
    }

    fn directed_path(n: u32) -> AdjMap<u32> {
        AdjMap::from_edges((0..n).tuple_windows())
    }

    fn transitive_tournament(n: u32) -> AdjMap<u32> {
        AdjMap::from_edges((0..n).tuple_combinations())
    }
}

impl std::str::FromStr for AdjMap<u32> {
    type Err = NotRegistered;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(g) = s.chars().next() {
            if let Ok(n) = &s[1..].parse::<u32>() {
                match g {
                    'k' => return Ok(AdjMap::complete_graph(*n)),
                    'c' => return Ok(AdjMap::directed_cycle(*n)),
                    'p' => return Ok(AdjMap::directed_path(*n)),
                    't' => return Ok(AdjMap::transitive_tournament(*n)),
                    _ => return Err(NotRegistered),
                }
            }
        }
        Err(NotRegistered)
    }
}

#[derive(Debug)]
pub struct NotRegistered;

impl std::fmt::Display for NotRegistered {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No graph registered with that name")
    }
}

impl std::error::Error for NotRegistered {}
#[cfg(test)]
mod test {
    // use super::*;
}
