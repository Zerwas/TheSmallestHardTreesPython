use std::error::Error;
use std::io::{Read, Write};
use std::{fmt, io, num};

use itertools::Itertools;

use super::classes::Buildable;
use super::traits::Digraph;

/// Prints the graph in dot format.
pub fn to_dot<'a, G, W>(g: &'a G, output: &mut W) -> Result<(), io::Error>
where
    G::Vertex: fmt::Display,
    G: Digraph<'a>,
    W: Write,
{
    let mut s = String::from("digraph {\n");
    for v in g.vertices() {
        s.push_str(&format!("\"{}\";\n", v));
    }
    for (u, v) in g.edges() {
        s.push_str(&format!("\"{}\" -> \"{}\";\n", u, v));
    }
    s.push('}');
    output.write_all(s.as_bytes())?;

    Ok(())
}

/// Prints the graph in dot format.
pub fn to_edge_list<'a, G, W>(g: &'a G, output: &mut W) -> Result<(), io::Error>
where
    G::Vertex: fmt::Display,
    G: Digraph<'a>,
    W: Write,
{
    let mut s = String::from("[");
    for (i, (u, v)) in g.edges().enumerate() {
        if i != 0 {
            s.push(',');
        }
        s.push_str(&format!("({},{})", u, v));
    }
    s.push(']');
    output.write_all(s.as_bytes())?;

    Ok(())
}

// /// Prints the graph in csv format.
// pub fn to_csv<G, W>(g: &G, output: &mut W) -> Result<(), io::Error> {
//     let bytes = g
//         .edges()
//         .flat_map(|(u, v)| format!("{:?};{:?}\n", u, v).into_bytes())
//         .collect_vec();
//     output.write_all(&bytes)?;

//     Ok(())
// }

/// Reads a graph from csv format.
pub fn from_csv<G, R>(mut read: R) -> Result<G, CsvError>
where
    G: Buildable<Vertex = usize>,
    R: Read,
{
    let mut content = String::new();
    read.read_to_string(&mut content)?;
    let mut edges = Vec::new();

    for (i, line) in content.lines().enumerate() {
        if let Some((v, w)) = line.split(&[',', ';', '|', ' ']).next_tuple() {
            edges.push((v.parse::<G::Vertex>()?, w.parse::<usize>()?));
        } else {
            return Err(CsvError::MissingSeparator(i + 1));
        }
    }

    let mut g = G::with_capacities(edges.len() * 2, edges.len());
    for (u, v) in edges {
        g.add_edge(u, v);
    }
    g.shrink_to_fit();

    Ok(g)
}

pub fn from_edge_list<G: FromIterator<(usize, usize)>>(s: &str) -> G {
    G::from_iter(
        s.split(&['[', ']', ',', '(', ')', '"'])
            .filter(|&x| !x.is_empty())
            .tuples()
            .map(|(u, v)| (u.parse::<usize>().unwrap(), v.parse::<usize>().unwrap())),
    )
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
