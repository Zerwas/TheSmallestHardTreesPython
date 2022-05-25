use std::fmt::Debug;

pub trait Buildable {
    type Vertex: Copy;

    fn with_capacities(nvertices: usize, nedges: usize) -> Self;

    fn shrink_to_fit(&mut self) {}

    fn add_vertex(&mut self) -> Self::Vertex;

    fn add_edge(&mut self, u: Self::Vertex, v: Self::Vertex);
}

/// Returns a path with `m` edges.
///
/// The path is directed if G is a digraph.
pub fn directed_path<G>(m: usize) -> G
where
    G: Buildable,
{
    let mut g = G::with_capacities(m + 1, m);
    let nodes: Vec<_> = (0..=m).map(|_| g.add_vertex()).collect();
    for (u, v) in nodes.iter().zip(nodes.iter().skip(1)) {
        g.add_edge(*u, *v);
    }
    g
}

/// Returns a cycle with length `n`.
///
/// The cycle is directed if G is directed
pub fn directed_cycle<G>(n: usize) -> G
where
    G: Buildable,
    G::Vertex: Debug,
{
    let mut g = G::with_capacities(n, n);
    let nodes: Vec<_> = (0..n).map(|_| g.add_vertex()).collect();
    for (u, v) in nodes.iter().zip(nodes.iter().cycle().skip(1)) {
        g.add_edge(*u, *v);
    }
    g
}

/// Returns a transitive tournament graph with `n` nodes.
pub fn transitive_tournament<G>(n: usize) -> G
where
    G: Buildable,
{
    let mut g = G::with_capacities(n, n * (n - 1) / 2);
    let nodes: Vec<_> = (0..n).map(|_| g.add_vertex()).collect();
    for (i, &u) in nodes.iter().enumerate() {
        for &v in &nodes[i + 1..] {
            g.add_edge(u, v);
        }
    }
    g
}

/// Returns the complete graph on `n` nodes.
pub fn complete_digraph<G>(n: usize) -> G
where
    G: Buildable,
{
    let mut g = G::with_capacities(n, n * (n - 1) / 2);
    let nodes: Vec<_> = (0..n).map(|_| g.add_vertex()).collect();
    for (i, &u) in nodes.iter().enumerate() {
        for &v in &nodes[i + 1..] {
            g.add_edge(u, v);
            g.add_edge(v, u);
        }
    }
    g
}
