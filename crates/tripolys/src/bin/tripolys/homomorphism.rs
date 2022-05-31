use std::{collections::HashMap, fmt::Debug, num::ParseIntError};

use arx::solver::BTSolver;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use colored::Colorize;
use tripolys::hcoloring::Instance;

use crate::{parse_graph, print_stats, CmdResult};

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("homomorphism")
        .setting(AppSettings::DeriveDisplayOrder)
        .about("Check for a homomorphism from G to H")
        .arg(
            Arg::with_name("from")
                .short("f")
                .long("from")
                .value_name("G")
                .takes_value(true)
                .required(true)
                .help("Check for homomorphism from graph G..."),
        )
        .arg(
            Arg::with_name("to")
                .short("t")
                .long("to")
                .value_name("H")
                .takes_value(true)
                .required(true)
                .help("...to graph H"),
        )
        .arg(
            Arg::with_name("precolor")
                .short("p")
                .long("precolor")
                .value_name("FILE")
                .takes_value(true)
                .conflicts_with("lists")
                .help("...with precoloring for some vertices [each line holding v:p(v)]"),
        )
        .arg(
            Arg::with_name("lists")
                .short("l")
                .long("lists")
                .value_name("FILE")
                .takes_value(true)
                .conflicts_with("precolor")
                .help("...with lists for each vertex v [line i holding l(i) given by comma-seperated values]"),
        )
}

pub fn command(args: &ArgMatches) -> CmdResult {
    let g = parse_graph(args.value_of("from").unwrap())?;
    let h = parse_graph(args.value_of("to").unwrap())?;

    let problem = if let Some(pc) = args.value_of("precolor") {
        let content = std::fs::read_to_string(pc)?;
        let pc = parse_precoloring(&content)?;
        Instance::with_precoloring(g, h, |v| pc.get(&v).copied())
    } else if let Some(lts) = args.value_of("lists") {
        let content = std::fs::read_to_string(lts)?;
        let lists = parse_lists(&content)?;
        Instance::with_lists(g, h, |v| lists[v].clone())
    } else {
        Instance::new(g, h)
    };

    println!("\n> Checking for homomorphism...");
    let mut solver = BTSolver::new(&problem);

    if solver.solution_exists() {
        println!("{}", "  ✓ Exists\n".to_string().green());
    } else {
        println!("{}", "  ! Doesn't exist\n".to_string().red());
    };

    if let Some(stats) = solver.stats() {
        print_stats(stats)
    }

    Ok(())
}

fn parse_lists(s: &str) -> Result<Vec<Vec<usize>>, ParseIntError> {
    s.lines()
        .map(|l| {
            l.split(',')
                .map(|v| v.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
}

fn parse_precoloring(s: &str) -> Result<HashMap<usize, usize>, ParsePrecoloringError> {
    use self::ParsePrecoloringError::*;

    s.lines()
        .map(|l| {
            l.split_once(':').map(|(a, b)| {
                a.parse::<usize>()
                    .and_then(|u| b.parse::<usize>().map(|v| (u, v)))
                    .map_err(ParseVertex)
            })
        })
        .collect::<Option<Result<HashMap<_, _>, _>>>()
        .ok_or(MissingDelimiter)?
}

#[derive(Debug)]
pub enum ParsePrecoloringError {
    MissingDelimiter,
    ParseVertex(ParseIntError),
}

impl std::fmt::Display for ParsePrecoloringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParsePrecoloringError::MissingDelimiter => write!(f, "Missing delimiter: ':'"),
            ParsePrecoloringError::ParseVertex(e) => std::fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for ParsePrecoloringError {}
