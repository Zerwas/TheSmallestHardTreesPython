pub trait GraphType {
    type Vertex: Copy + Eq;
}

pub trait Vertices<'a>: GraphType {
    type VertexIter: Iterator<Item = Self::Vertex>;

    fn vertices(&'a self) -> Self::VertexIter;

    fn vertex_count(&self) -> usize;

    fn has_vertex(&self, v: Self::Vertex) -> bool;
}

pub trait Edges<'a>: GraphType {
    type EdgeIter: Iterator<Item = (Self::Vertex, Self::Vertex)>;

    fn edges(&'a self) -> Self::EdgeIter;

    fn edge_count(&self) -> usize;

    fn has_edge(&self, u: Self::Vertex, v: Self::Vertex) -> bool;
}

pub trait GraphSize<'a>: Vertices<'a> + Edges<'a> {}

pub trait Digraph<'a>: Vertices<'a> + Edges<'a> {}
