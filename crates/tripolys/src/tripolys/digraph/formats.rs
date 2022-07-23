use std::io::{Read, Write};
use std::{fmt, io, num};

use itertools::Itertools;
use thiserror::Error;

use super::classes::Buildable;
use super::traits::{Digraph, Edges};

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
        s.push_str(&format!("({};{})", u, v));
    }
    s.push(']');
    output.write_all(s.as_bytes())?;

    Ok(())
}

/// Prints the graph in csv format.
pub fn to_csv<'a, G, W>(g: &'a G, output: &mut W) -> Result<(), io::Error>
where
    G: Edges<'a>,
    G::Vertex: std::fmt::Display,
    W: std::io::Write,
{
    let bytes = g
        .edges()
        .flat_map(|(u, v)| format!("{};{}\n", u, v).into_bytes())
        .collect_vec();
    output.write_all(&bytes)?;

    Ok(())
}

/// Reads a graph from csv format.
pub fn from_csv<G, R>(mut read: R) -> Result<G, CsvError>
where
    G: Buildable<Vertex = usize> + fmt::Debug,
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
    for _ in 0..=edges.len() {
        g.add_vertex();
    }
    for (u, v) in edges {
        g.add_edge(u, v);
    }
    g.shrink_to_fit();

    Ok(g)
}

pub fn from_edge_list<G: FromIterator<(usize, usize)>>(s: &str) -> G {
    s.split(&['[', ']', ',', '(', ')', '"'])
        .filter(|&x| !x.is_empty())
        .tuples()
        .map(|(u, v)| (u.parse::<usize>().unwrap(), v.parse::<usize>().unwrap()))
        .collect()
}

#[derive(Error, Debug)]
pub enum CsvError {
    #[error("Separator missing in line {0}")]
    MissingSeparator(usize),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Parse(#[from] num::ParseIntError),
}
