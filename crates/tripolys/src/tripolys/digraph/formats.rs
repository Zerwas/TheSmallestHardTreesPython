use std::error::Error;
use std::path::Path;
use std::{io, num};

use itertools::Itertools;

use super::{AdjMap, VertexId};

/// Prints the graph in dot format.
pub fn to_dot<V: VertexId>(
    g: &AdjMap<V>,
    output: &mut impl std::io::Write,
) -> Result<(), io::Error> {
    let mut s = String::from("digraph {\n");
    for v in g.vertices() {
        s.push_str(&format!("\"{:?}\";\n", v));
    }
    for (u, v) in g.edges() {
        s.push_str(&format!("\"{:?}\" -> \"{:?}\";\n", u, v));
    }
    s.push('}');
    output.write_all(s.as_bytes())?;

    Ok(())
}

/// Prints the graph in csv format.
pub fn to_csv<V: VertexId>(
    g: &AdjMap<V>,
    output: &mut impl std::io::Write,
) -> Result<(), io::Error> {
    let bytes = g
        .edges()
        .flat_map(|(u, v)| format!("{:?};{:?}\n", u, v).into_bytes())
        .collect_vec();
    output.write_all(&bytes)?;

    Ok(())
}

/// Reads a graph from csv format.
pub fn from_csv<P: AsRef<Path>>(path: P) -> Result<AdjMap<u32>, CsvError> {
    let content = std::fs::read_to_string(path)?;
    let mut edges = Vec::new();

    for (i, line) in content.lines().enumerate() {
        if let Some((v, w)) = line.split(&[',', ';', '|', ' ']).next_tuple() {
            edges.push((v.parse::<u32>()?, w.parse::<u32>()?));
        } else {
            return Err(CsvError::MissingSeparator(i + 1));
        }
    }
    Ok(AdjMap::from_edges(edges))
}

pub fn from_edge_list(s: &str) -> AdjMap<u32> {
    let edges = s
        .split(&['[', ']', ',', '(', ')', '"'])
        .filter(|&x| !x.is_empty())
        .tuples()
        .map(|(u, v)| (u.parse::<u32>().unwrap(), v.parse::<u32>().unwrap()));
    AdjMap::from_edges(edges)
}

impl std::fmt::Display for CsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CsvError::MissingSeparator(i) => write!(f, "Separator missing in line {}", i),
            CsvError::Io(ref err) => err.fmt(f),
            CsvError::Parse(ref err) => err.fmt(f),
        }
    }
}

impl From<num::ParseIntError> for CsvError {
    fn from(err: num::ParseIntError) -> CsvError {
        CsvError::Parse(err)
    }
}

impl From<io::Error> for CsvError {
    fn from(err: io::Error) -> CsvError {
        CsvError::Io(err)
    }
}

impl Error for CsvError {}

#[derive(Debug)]
pub enum CsvError {
    MissingSeparator(usize),
    Io(io::Error),
    Parse(num::ParseIntError),
}
