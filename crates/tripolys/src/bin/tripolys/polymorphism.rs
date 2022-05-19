use arx::solver::{BTSolver, BTStats};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use colored::*;
use csv::WriterBuilder;
use rayon::prelude::*;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use time::Duration;
use tripolys::tree::{Triad, Tree, Node};

use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;

use tripolys::algebra::conditions::*;
use tripolys::algebra::MetaProblem;
use tripolys::digraph::{from_edge_list, AdjMap, ToGraph};

use crate::{parse_graph, CmdResult};

const AVAILABLE_CONDITIONS: [&str; 8] = [
    "majority", "siggers", "kkm", "k-wnu", "k-nu", "k-sigma", "n-j", "n-hm",
];

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("polymorphism")
        .setting(AppSettings::DeriveDisplayOrder)
        .about("Study the polymorphisms of finite digraphs")
        .arg(
            Arg::with_name("idempotent")
                .short("I")
                .long("idempotent")
                .help("Require idempotence"),
        )
        .arg(
            Arg::with_name("conservative")
                .short("C")
                .long("conservative")
                .help("Require conservativity"),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("List the available conditions"),
        )
        .arg(
            Arg::with_name("condition")
                .short("c")
                .long("condition")
                .takes_value(true)
                .value_name("NAME")
                .help("The name of the condition the polymorphism must satisfy")
                .required_unless("list"),
        )
        .arg(
            Arg::with_name("balanced")
                .short("b")
                .long("balanced")
                .help("Optimize based on the promise that H is balanced"),
        )
        .arg(
            Arg::with_name("graph")
                .short("g")
                .long("graph")
                .takes_value(true)
                .value_name("H")
                .help("Check for polymorphisms of single graph H"),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .requires("output")
                .takes_value(true)
                .value_name("FILE")
                .help("The file from where to read in graphs"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .requires("input")
                .takes_value(true)
                .value_name("FILE")
                .help("The file to which the results are written"),
        )
        .arg(
            Arg::with_name("filter")
                .short("f")
                .long("filter")
                .requires("input")
                .takes_value(true)
                .value_name("PREDICATE")
                .possible_values(&["deny", "admit"])
                .help("Only write graphs to results if they deny/admit a polymorphism"),
        )
}

pub fn command(args: &ArgMatches) -> CmdResult {
    if args.is_present("list") {
        println!("\nAvailable conditions:");
        for condition in &AVAILABLE_CONDITIONS {
            println!(" - {}", condition);
        }
        return Ok(());
    }

    let condition = args.value_of("condition").unwrap();
    let conservative = args.is_present("conservative");
    let idempotent = args.is_present("idempotent");

    if let (Some(input_path), Some(output_path)) = (args.value_of("input"), args.value_of("output"))
    {
        let mut graphs = Vec::new();
        let lines = std::fs::read_to_string(input_path)?;
        let mut lines = lines.lines();

        if input_path.ends_with(".csv") {
            let _ = lines.next();
        }
        for line in lines {
            if let Some(s) = line.split(';').next() {
                graphs.push(from_edge_list(s));
            }
        }

        let log = std::sync::Mutex::new(SearchLog::new());

        println!("  > Checking for polymorphism...",);
        let start = std::time::Instant::now();

        graphs.into_par_iter().for_each(|item| {
            let problem = create_meta_problem(&item.to_graph(), condition).unwrap();
            let mut solver = BTSolver::new(&problem);
            let found = solver.solution_exists();

            let write = if let Some(filter) = args.value_of("filter") {
                match filter {
                    "deny" => !found,
                    "admit" => found,
                    &_ => {
                        unreachable!()
                    }
                }
            } else {
                true
            };

            if write {
                log.lock()
                    .unwrap()
                    .add(item, found, solver.stats().unwrap());
            }
        });
        println!("    - total_time: {:?}", start.elapsed());

        println!("  > Writing results...",);
        log.lock().unwrap().write_csv(&output_path)?;

        return Ok(());
    }

    // let h = parse_graph(args.value_of("graph").unwrap())?;
    // let h = Triad::from_str(args.value_of("graph").unwrap())?;
    let h = Node::from_str(args.value_of("graph").unwrap())?;
    let mut problem = create_meta_problem_tree(&h, condition)?;
    if conservative {
        problem.conservative();
    }
    if idempotent {
        problem.idempotent();
    }
    let mut solver = BTSolver::new(&problem);

    println!("\n> Checking for polymorphism...");

    if solver.solution_exists() {
        println!("{}", "  ✓ Exists\n".to_string().green());
    } else {
        println!("{}", "  \u{2718} Doesn't exist\n".to_string().red());
    };

    if let Some(stats) = solver.stats() {
        stats.print();
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseConditionError;

impl std::fmt::Display for ParseConditionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No condition registered with that name")
    }
}

impl std::error::Error for ParseConditionError {}

fn create_meta_problem(h: &AdjMap<u32>, s: &str) -> Result<MetaProblem<u32>, ParseConditionError> {
    match s {
        "majority" => Ok(MetaProblem::new(h, Majority)),
        "siggers" => Ok(MetaProblem::new(h, Siggers)),
        "kmm" => Ok(MetaProblem::new(h, Kmm)),
        _ => {
            if let Some((pr, su)) = s.split_once('-') {
                if let Ok(pr) = pr.parse() {
                    match su {
                        "wnu" => Ok(MetaProblem::new(h, Wnu(pr))),
                        "nu" => Ok(MetaProblem::new(h, Nu(pr))),
                        "sigma" => Ok(MetaProblem::new(h, Sigma(pr))),
                        "j" => Ok(MetaProblem::new(h, Jonsson(pr))),
                        // TODO "hm" => Ok(MetaProblem::new(h, Hm(n))),
                        "kk" => Ok(MetaProblem::new(h, KearnesKiss(pr))),
                        // "noname" => Ok(MetaProblem::new(h, Noname(pr))),
                        &_ => Err(ParseConditionError),
                    }
                } else {
                    Err(ParseConditionError)
                }
            } else {
                Err(ParseConditionError)
            }
        }
    }
}

fn create_meta_problem_tree<T: Tree>(t: &T, s: &str) -> Result<MetaProblem<u32>, ParseConditionError> {
    match s {
        "majority" => Ok(MetaProblem::<u32>::from_tree(t, Majority)),
        "siggers" => Ok(MetaProblem::<u32>::from_tree(t, Siggers)),
        "kmm" => Ok(MetaProblem::<u32>::from_tree(t, Kmm)),
        _ => {
            if let Some((pr, su)) = s.split_once('-') {
                if let Ok(pr) = pr.parse() {
                    match su {
                        "wnu" => Ok(MetaProblem::<u32>::from_tree(t, Wnu(pr))),
                        "nu" => Ok(MetaProblem::<u32>::from_tree(t, Nu(pr))),
                        "sigma" => Ok(MetaProblem::<u32>::from_tree(t, Sigma(pr))),
                        "j" => Ok(MetaProblem::<u32>::from_tree(t, Jonsson(pr))),
                        // TODO "hm" => Ok(MetaProblem::<u32>::from_tree(h, Hm(n))),
                        "kk" => Ok(MetaProblem::<u32>::from_tree(t, KearnesKiss(pr))),
                        // "noname" => Ok(MetaProblem::new(h, Noname(pr))),
                        &_ => Err(ParseConditionError),
                    }
                } else {
                    Err(ParseConditionError)
                }
            } else {
                Err(ParseConditionError)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No condition registered with name")
    }
}

impl std::error::Error for Error {}

/// A struct which allows to store recorded data during the polymorphism search.
#[derive(Debug, Default)]
struct Record {
    /// String representation of the inspected tree
    tree: String,
    /// Whether the polymorphism was found
    found: bool,
    /// Number of times the search-algorithm backtracked
    backtracks: u32,
    /// Time it took for the initial run of arc-consistency
    ac3_time: Duration,
    /// Time it took for the backtracking-search
    mac3_time: Duration,
    /// Total sum of ac_time and search_time
    total_time: Duration,
}

impl Record {
    fn new(tree: impl Display, found: bool, stats: &BTStats) -> Record {
        Record {
            tree: tree.to_string(),
            found,
            backtracks: stats.backtracks,
            ac3_time: stats.ac3_time,
            mac3_time: stats.mac3_time,
            total_time: stats.ac3_time + stats.mac3_time,
        }
    }
}

impl Serialize for Record {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Record", 6)?;
        state.serialize_field("tree", &self.tree)?;
        state.serialize_field("found", &self.found)?;
        state.serialize_field("backtracks", &self.backtracks)?;
        state.serialize_field("ac3_time", &format!("{:?}", self.ac3_time))?;
        state.serialize_field("mac3_time", &format!("{:?}", self.mac3_time))?;
        state.serialize_field("total_time", &format!("{:?}", self.total_time))?;
        state.end()
    }
}

/// Stores stats and prints them to csv.
#[derive(Debug, Default)]
pub struct SearchLog(Vec<Record>);

impl SearchLog {
    pub fn new() -> SearchLog {
        SearchLog::default()
    }

    pub fn add(&mut self, tree: impl Display, found: bool, stats: &BTStats) {
        self.0.push(Record::new(tree, found, stats));
    }

    pub fn write_csv<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {
        let mut wtr = WriterBuilder::new()
            .has_headers(true)
            .delimiter(b';')
            .from_path(&path)?;
        for record in &self.0 {
            wtr.serialize(record)?;
        }
        Ok(())
    }
}
