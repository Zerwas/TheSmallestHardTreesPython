//! # Tripolys
//!
//! `tripolys` is a program for checking homomorphisms and testing polymorphism
//! conditions of directed graphs. It also implements an algorithm to generate
//! orientations of trees, and core orientations of trees.

use std::error::Error;
use std::fmt::Debug;

use arx::solver::SolveStats;
use clap::{App, AppSettings};
use colored::*;
use tripolys::digraph::{classes::*, formats::from_csv, AdjMatrix};

mod dot;
mod endomorphism;
mod generate;
mod homomorphism;
mod polymorphism;

type CmdResult = Result<(), Box<dyn Error>>;

pub fn cli() -> App<'static, 'static> {
    App::new("Tripolys")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::VersionlessSubcommands)
        .author("Michael W. <michael.wernthaler@posteo.de>")
        .about("A program for checking homomorphisms and testing polymorphism conditions of directed graphs.")
        .subcommands([
            endomorphism::cli(),
            homomorphism::cli(),
            polymorphism::cli(),
            generate::cli(),
            dot::cli(),
        ])
}

/// Print error message to stderr and terminate
fn error(message: &str) -> ! {
    eprintln!("{} {}", "error:".red().bold(), message);
    std::process::exit(1);
}

fn main() {
    let args = cli().get_matches();

    let result = match args.subcommand() {
        ("endomorphism", Some(matches)) => endomorphism::command(matches),
        ("homomorphism", Some(matches)) => homomorphism::command(matches),
        ("polymorphism", Some(matches)) => polymorphism::command(matches),
        ("dot", Some(matches)) => dot::command(matches),
        ("generate", Some(matches)) => generate::command(matches),
        _ => Ok(()),
    };

    match result {
        Err(e) => error(&e.to_string()),
        Ok(()) => {}
    }
}

fn parse_graph(s: &str) -> Result<AdjMatrix, Box<dyn Error>> {
    if let Ok(class) = parse_class(s) {
        return Ok(class);
    }
    if let Ok(triad) = parse_triad(s) {
        return Ok(triad);
    }
    if let Ok(mut file) = std::fs::File::open(s) {
        if let Ok(g) = from_csv(&mut file) {
            return Ok(g);
        }
    }
    Err(Box::new(ParseGraphError))
}

fn parse_class<G>(s: &str) -> Result<G, ClassNotFound>
where
    G: Buildable,
    G::Vertex: Debug,
{
    if let Some(g) = s.chars().next() {
        if let Ok(n) = &s[1..].parse::<usize>() {
            match g {
                'k' => return Ok(complete_digraph(*n)),
                'c' => return Ok(directed_cycle(*n)),
                'p' => return Ok(directed_path(*n)),
                't' => return Ok(transitive_tournament(*n)),
                _ => return Err(ClassNotFound),
            }
        }
    }
    Err(ClassNotFound)
}

#[derive(Debug)]
pub struct ParseGraphError;

impl std::fmt::Display for ParseGraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Could not parse graph from the given argument")
    }
}

impl std::error::Error for ParseGraphError {}

#[derive(Debug)]
pub struct ClassNotFound;

impl std::fmt::Display for ClassNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No graph class registered with that name")
    }
}

impl std::error::Error for ClassNotFound {}

pub fn parse_triad<G>(s: &str) -> Result<G, ParseTriadError>
where
    G: Buildable + std::fmt::Debug,
    G::Vertex: std::fmt::Debug,
{
    if s.matches(',').count() != 2 {
        return Err(ParseTriadError::NumArms);
    }

    let nvertices = s.len() - 1;
    let mut g = G::with_capacities(nvertices, nvertices - 1);
    let vertices: Vec<_> = (0..nvertices).map(|_| g.add_vertex()).collect();
    let mut j = 1;

    for arm in s.split(',') {
        for (i, c) in arm.chars().enumerate() {
            match c {
                '1' => {
                    if i == 0 {
                        g.add_edge(vertices[j], vertices[0]);
                    } else {
                        g.add_edge(vertices[j], vertices[j - 1]);
                    }
                }
                '0' => {
                    if i == 0 {
                        g.add_edge(vertices[0], vertices[j]);
                    } else {
                        g.add_edge(vertices[j - 1], vertices[j]);
                    }
                }
                c => {
                    return Err(ParseTriadError::InvalidCharacter(c));
                }
            }
            j += 1;
        }
    }

    Ok(g)
}

#[derive(Debug)]
pub enum ParseTriadError {
    NumArms,
    InvalidCharacter(char),
}

impl std::fmt::Display for ParseTriadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseTriadError::NumArms => write!(f, "A triad must have exactly 3 arms!"),
            ParseTriadError::InvalidCharacter(c) => write!(f, "Could not parse: {}", c),
        }
    }
}

impl std::error::Error for ParseTriadError {}

fn print_stats(ss: &SolveStats) {
    println!("Statistics:");
    println!("- {: <12} {}", "#ccks:", ss.ccks);
    println!("- {: <12} {}", "#backtracks:", ss.backtracks);
    println!("- {: <12} {:?}s", "ac3_time:", ss.ac3_time.as_secs_f32());
    println!("- {: <12} {:?}s", "mac3_time:", ss.mac3_time.as_secs_f32());
}
