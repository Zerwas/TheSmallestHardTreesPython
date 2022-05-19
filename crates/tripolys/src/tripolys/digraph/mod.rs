//! A set of vertices connected by directed edges.

// TODO often called arcs.

mod adj_map;
mod adj_matrix;
mod formats;
mod levels;

pub use adj_map::AdjMap;
pub use adj_map::ToGraph;
pub use adj_map::VertexId;
pub use adj_matrix::AdjMatrix;

pub use formats::from_csv;
pub use formats::from_edge_list;
pub use formats::to_csv;
pub use formats::to_dot;
pub use formats::CsvError;

pub use levels::levels;
