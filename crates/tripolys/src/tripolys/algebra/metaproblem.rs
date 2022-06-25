use bimap::BiMap;
use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

use crate::digraph::classes::Buildable;
use crate::digraph::traits::{Edges, Vertices};
use crate::digraph::{levels, AdjMatrix};
use crate::hcoloring::Instance;

use super::IterAlgebra;

type Partition<V> = Vec<Vec<V>>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Condition {
    Kmm,
    Siggers,
    Majority,
    Nu(usize),
    Wnu(usize),
    NoName(usize),
    Jonsson(usize),
    KearnesKiss(usize),
    HobbyMcKenzie(usize),
    HagemannMitschke(usize),
}

impl FromStr for Condition {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match &*s.to_ascii_lowercase() {
            "majority" => Ok(Condition::Majority),
            "siggers" => Ok(Condition::Siggers),
            "kmm" => Ok(Condition::Kmm),
            _ => {
                if let Some((pr, su)) = s.split_once('-') {
                    if let Ok(pr) = pr.parse() {
                        match su {
                            "wnu" => Ok(Condition::Wnu(pr)),
                            "nu" => Ok(Condition::Nu(pr)),
                            "j" => Ok(Condition::Jonsson(pr)),
                            "hm" => Ok(Condition::HagemannMitschke(pr)),
                            "kk" => Ok(Condition::KearnesKiss(pr)),
                            "hmck" => Ok(Condition::HobbyMcKenzie(pr)),
                            "nn" => Ok(Condition::NoName(pr)),
                            &_ => Err("unknown Condition, cannot convert from str".to_owned()),
                        }
                    } else {
                        Err("unknown Condition, cannot convert from str".to_owned())
                    }
                } else {
                    Err("unknown Condition, cannot convert from str".to_owned())
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseConditionError;

impl std::fmt::Display for ParseConditionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No condition registered with that name")
    }
}

impl std::error::Error for ParseConditionError {}

/// The problem of deciding whether a graph has a given type of polymorphism(s).
#[derive(Clone, Copy, Debug)]
pub struct MetaProblem {
    condition: Condition,
    level_wise: bool,
    conservative: bool,
    idempotent: bool,
}

impl MetaProblem {
    pub fn new(condition: Condition) -> MetaProblem {
        MetaProblem {
            level_wise: true,
            conservative: false,
            idempotent: false,
            condition,
        }
    }

    pub fn level_wise(mut self, flag: bool) -> Self {
        self.level_wise = flag;
        self
    }

    pub fn conservative(mut self, flag: bool) -> Self {
        self.conservative = flag;
        self
    }

    pub fn idempotent(mut self, flag: bool) -> Self {
        self.idempotent = flag;
        self
    }

    pub fn instance(self, h: &AdjMatrix) -> Result<Instance, Error> {
        let condition = self.condition;
        let levels = if self.level_wise {
            levels(h).ok_or(Error::Unbalanced)?
        } else {
            vec![]
        };
        // Indicator graph construction
        let mut indicator_graph = arities(condition)
            .into_iter()
            .enumerate()
            .flat_map(|(i, k)| h.edges().power(k).map(move |(u, v)| ((i, u), (i, v))))
            .filter(|((_, u), _)| !self.level_wise || u.iter().map(|v| levels[*v]).all_equal())
            .collect::<AdjMap<_>>();

        for class in eq_classes(condition, &h.vertices().collect_vec()) {
            for i in 1..class.len() {
                indicator_graph.contract_vertices(&class[0], &class[i]);
            }
        }

        let mut indicator_matrix = AdjMatrix::with_capacities(
            indicator_graph.vertex_count(),
            indicator_graph.edge_count(),
        );
        let id_map = indicator_graph
            .vertices()
            .cloned()
            .map(|v| (v, indicator_matrix.add_vertex()))
            .collect::<BiMap<_, _>>();

        for (u, v) in indicator_graph.edges() {
            indicator_matrix.add_edge(
                *id_map.get_by_left(&u).unwrap(),
                *id_map.get_by_left(&v).unwrap(),
            );
        }
        // Indicator graph construction
        let instance = Instance::with_lists(indicator_matrix, h.clone(), |v| {
            let vertex = id_map.get_by_right(&v).unwrap();

            if let Some(u) = precolor(condition, vertex) {
                vec![u]
            } else if self.conservative {
                vertex.1.to_vec()
            } else if self.idempotent {
                if vertex.1.iter().all_equal() {
                    vec![vertex.1[0]]
                } else {
                    h.vertices().collect()
                }
            } else {
                h.vertices().collect()
            }
        });

        Ok(instance)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Unbalanced,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Unbalanced => write!(f, "The given graph is not balanced"),
        }
    }
}

impl std::error::Error for Error {}

fn arities(condition: Condition) -> Vec<usize> {
    match condition {
        Condition::Kmm => vec![3, 3],
        Condition::Siggers => vec![4],
        Condition::Majority => vec![3],
        Condition::Nu(k) => vec![k],
        Condition::Wnu(k) => vec![k],
        Condition::NoName(n) => vec![4; n],
        Condition::Jonsson(n) => vec![3; n],
        Condition::KearnesKiss(n) => vec![3; n],
        Condition::HobbyMcKenzie(n) => vec![3; n],
        Condition::HagemannMitschke(n) => vec![3; n],
    }
}

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

fn precolor(condition: Condition, (f, v): &(usize, Vec<usize>)) -> Option<usize> {
    match condition {
        Condition::Majority => {
            if v.iter().all_equal() {
                Some(v[0])
            } else {
                None
            }
        }
        Condition::Nu(_) => {
            if let ElemCount::Once(x1, _) = elem_count(v) {
                Some(x1)
            } else {
                None
            }
        }
        Condition::NoName(n) => {
            if *f == 0 && v[1] == v[2] {
                return Some(v[0]);
            }
            if *f == n && v[0] == v[1] {
                return Some(v[3]);
            }
            None
        }
        Condition::HobbyMcKenzie(n) => {
            if *f == 0 {
                return Some(v[0]);
            }
            if *f == (2 * n + 2) {
                return Some(v[2]);
            }
            None
        }
        Condition::HagemannMitschke(n) => {
            if *f == 0 && v[1] == v[2] {
                return Some(v[0]);
            }
            if *f == (n - 1) && v[0] == v[1] {
                return Some(v[2]);
            }
            None
        }
        _ => None,
    }
}

fn eq_classes(condition: Condition, vertices: &[usize]) -> Partition<(usize, Vec<usize>)> {
    match condition {
        Condition::Kmm => {
            let mut partition = Vec::new();

            for &x in vertices {
                for &y in vertices {
                    if x == y {
                        partition.push(vec![(0, vec![x, x, x]), (1, vec![x, x, x])]);
                    }
                    partition.push(vec![
                        (0, vec![x, y, y]),
                        (1, vec![y, x, x]),
                        (1, vec![x, x, y]),
                    ]);
                    partition.push(vec![(0, vec![x, y, x]), (1, vec![x, y, x])]);
                }
            }

            partition
        }
        Condition::Siggers => {
            let mut vec = Vec::new();

            for &x in vertices {
                for &y in vertices {
                    for &z in vertices {
                        if x != y || y != z {
                            if y == z {
                                vec.push(vec![
                                    (0, vec![x, y, z, x]),
                                    (0, vec![y, x, y, z]),
                                    (0, vec![x, z, x, y]),
                                ]);
                            } else if x != z {
                                vec.push(vec![(0, vec![x, y, z, x]), (0, vec![y, x, y, z])]);
                            }
                        }
                    }
                }
            }
            vec
        }
        Condition::Majority => majority_eq_classes(vertices),
        Condition::Nu(arity) => nu_eq_classes(vertices, arity),
        Condition::Wnu(arity) => wnu_eq_classes(vertices, arity),
        Condition::NoName(length) => {
            let mut partition = Vec::new();

            for &x in vertices {
                for &y in vertices {
                    for i in 0..length {
                        partition.push(vec![(i, vec![x, x, y, x]), (i + 1, vec![x, y, y, x])]);
                        partition.push(vec![(i, vec![x, x, y, y]), (i + 1, vec![x, y, y, y])]);
                    }
                }
            }

            partition
        }
        Condition::Jonsson(length) => {
            let mut partition = Vec::new();

            for &x in vertices {
                let mut id = (0..=(2 * length)).map(|i| (i, vec![x, x, x])).collect_vec();

                for &y in vertices {
                    if x == y {
                        continue;
                    }
                    for i in 0..length {
                        partition.push(vec![(2 * i, vec![x, y, y]), (2 * i + 1, vec![x, y, y])]);
                        partition
                            .push(vec![(2 * i + 1, vec![x, x, y]), (2 * i + 2, vec![x, x, y])]);
                    }
                    for i in 0..=(2 * length) {
                        id.push((i, vec![x, y, x]));
                    }
                    partition.push(vec![(0, vec![x, x, x]), (0, vec![x, x, y])]);
                    partition.push(vec![
                        (2 * length, vec![y, y, y]),
                        (2 * length, vec![x, y, y]),
                    ]);
                }
                partition.push(id);
            }

            partition
        }
        Condition::KearnesKiss(length) => {
            let mut partition = Vec::new();

            for &x in vertices {
                let mut id = (0..=length).map(|i| (i, vec![x, x, x])).collect_vec();

                for &y in vertices {
                    for i in (0..length).step_by(2) {
                        partition.push(vec![(i, vec![x, y, y]), (i + 1, vec![x, y, y])]);
                        partition.push(vec![(i, vec![x, y, x]), (i + 1, vec![x, y, x])]);
                    }
                    for i in (0..length).skip(1).step_by(2) {
                        partition.push(vec![(i, vec![x, x, y]), (i + 1, vec![x, x, y])]);
                    }
                    for &z in vertices {
                        id.push((0, vec![x, y, z]));
                        id.push((length, vec![y, z, x]));
                    }
                }
                partition.push(id);
            }

            partition
        }
        Condition::HobbyMcKenzie(n) => {
            let mut partition = Vec::new();

            for &x in vertices {
                partition.push((0..(2 * n + 3)).map(|i| (i, vec![x, x, x])).collect_vec());

                for &y in vertices {
                    if x == y {
                        continue;
                    }
                    partition.push(vec![(n, vec![x, y, y]), (n + 1, vec![x, y, y])]);
                    partition.push(vec![(n + 1, vec![x, x, y]), (n + 2, vec![x, x, y])]);

                    for j in (0..n).step_by(2) {
                        partition.push(vec![(j, vec![x, y, y]), (j + 1, vec![x, y, y])]);
                        partition
                            .push(vec![(j + n + 2, vec![x, y, y]), (j + n + 3, vec![x, y, y])]);
                        partition
                            .push(vec![(j + n + 2, vec![x, y, x]), (j + n + 3, vec![x, y, x])]);
                    }
                    for j in (0..n).skip(1).step_by(2) {
                        partition.push(vec![(j, vec![x, x, y]), (j + 1, vec![x, x, y])]);
                        partition.push(vec![(j, vec![x, y, x]), (j + 1, vec![x, y, x])]);
                        partition
                            .push(vec![(j + n + 2, vec![x, x, y]), (j + n + 3, vec![x, x, y])]);
                    }
                }
            }

            partition
        }
        Condition::HagemannMitschke(n) => {
            let mut partition = Vec::new();

            for &x in vertices {
                for &y in vertices {
                    for i in 0..n {
                        partition.push(vec![(i, vec![x, x, y]), (i + 1, vec![x, y, y])]);
                    }
                }
            }

            partition
        }
    }
}

fn wnu_eq_classes(g: &[usize], k: usize) -> Partition<(usize, Vec<usize>)> {
    nu_eq_class_helper(k, g, true)
}

fn nu_eq_classes(g: &[usize], k: usize) -> Partition<(usize, Vec<usize>)> {
    nu_eq_class_helper(k, g, false)
}

fn majority_eq_classes(g: &[usize]) -> Partition<(usize, Vec<usize>)> {
    nu_eq_class_helper(3, g, true)
}

fn nu_eq_class_helper(arity: usize, g: &[usize], weak: bool) -> Partition<(usize, Vec<usize>)> {
    let mut partition = Vec::new();

    for &v in g {
        let mut vec = Vec::new();

        for &w in g {
            if v == w {
                continue;
            }
            let mut eq_class = Vec::new();

            for k in 0..arity {
                let mut tuple = (0, vec![v; arity]);
                tuple.1[k] = w;
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

/// `AdjMap`<V> is a directed graph datastructure using an adjacency list
/// representation.
///
/// For each vertex the `HashMap` contains an ordered pair, the adjacency
/// lists, where the first entry and second entry contain all successors and
/// predecessors, respectively.
#[derive(Debug, Clone, Default)]
struct AdjMap<V> {
    // Vertex -> (Out-Edges, In-Edges)
    lists: IndexMap<V, (IndexSet<V>, IndexSet<V>)>,
    edges: IndexSet<(V, V)>,
}

impl<T> AdjMap<T> {
    /// Creates an empty `Graph`.
    pub fn new() -> AdjMap<T> {
        AdjMap {
            lists: IndexMap::new(),
            edges: IndexSet::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_capacities(nvertices: usize, nedges: usize) -> AdjMap<T> {
        AdjMap {
            lists: IndexMap::with_capacity(nvertices),
            edges: IndexSet::with_capacity(nedges),
        }
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
}

impl<T: Hash + Eq> AdjMap<T> {
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
            self.lists.insert(v, (IndexSet::new(), IndexSet::new()));
            true
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
}

impl<T: Hash + Eq + Clone> AdjMap<T> {
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
    #[allow(dead_code)]
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
                self.edges.remove::<_>(&(u.clone(), v.clone()));
            }
        }
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
                panic!("Vertex doesn't exist");
            }
            if let Some((_, in_edges)) = self.lists.get_mut(v) {
                in_edges.insert(u.clone());
            } else {
                panic!("Vertex doesn't exist");
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
    #[allow(dead_code)]
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
}

impl<V: Clone + Hash + Eq> FromIterator<(V, V)> for AdjMap<V> {
    fn from_iter<T: IntoIterator<Item = (V, V)>>(iter: T) -> AdjMap<V> {
        // TODO use with_capacity
        let mut graph = AdjMap::<V>::new();
        for (u, v) in iter {
            graph.add_vertex(u.clone());
            graph.add_vertex(v.clone());
            graph.add_edge(&u, &v);
        }
        graph
    }
}

impl<V: std::fmt::Display + Hash + Eq + Clone> std::fmt::Display for AdjMap<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("[");

        for (i, (u, v)) in self.edges().enumerate() {
            if i != 0 {
                s.push(',');
            }
            s.push_str(&format!("({},{})", u, v));
        }
        s.push(']');

        write!(f, "{}", s)
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
