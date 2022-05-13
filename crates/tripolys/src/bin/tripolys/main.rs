//! # Tripolys
//!
//! `tripolys` is a program for generating triads and checking for polymorphisms
//! on them.
//!
//! For a given digraph H the complexity of the constraint satisfaction problem
//! for H, also called CSP(H), only depends on the set of polymorphisms of H.
//! The program aims to study the structure of oriented trees with CSPs of
//! varying complexity.
//! To do this we focus on the case where H is a triad, e.g., an orientation of
//! a tree which has a single vertex of degree 3 and otherwise only vertices of
//! degree 2 and 1.

use std::error::Error;
use std::str::FromStr;

use clap::{App, AppSettings};
use colored::*;
use tripolys::digraph::AdjMap;
use tripolys::tree::{Node, Triad};

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
        .about("A program for studying graph colouring problems.")
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

fn parse_graph(s: &str) -> Result<AdjMap<u32>, Box<dyn Error>> {
    if s.len() < 4 {
        let g = AdjMap::from_str(s)?;
        return Ok(g);
    }
    if s.ends_with(".csv") {
        let g = tripolys::digraph::from_csv(s)?;
        return Ok(g);
    }
    if let Ok(triad) = s.parse::<Triad>() {
        return Ok(triad.into());
    }
    if let Ok(tree) = s.parse::<Node>() {
        return Ok(tree.into());
    }
    Err(Box::new(ParseGraphError))
}

#[derive(Debug)]
pub struct ParseGraphError;

impl std::fmt::Display for ParseGraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Could not parse graph from the given argument")
    }
}

impl std::error::Error for ParseGraphError {}
