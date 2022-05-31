use arx::solver::{BTSolver, SolveStats};
use clap::{App, AppSettings, Arg, ArgGroup, ArgMatches, SubCommand};
use colored::*;
use csv::WriterBuilder;
use rayon::prelude::*;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use tripolys::digraph::formats::from_edge_list;
use tripolys::digraph::AdjMatrix;

use std::fmt::Display;
use std::path::Path;
use std::time::Duration;

use tripolys::algebra::MetaProblem;
use tripolys::algebra::{conditions::*, Config};

use crate::{parse_graph, print_stats, CmdResult};

const AVAILABLE_CONDITIONS: [&str; 9] = [
    "majority    majority",
    "k-nu        k-ary near-unamity",
    "k-wnu       k-ary weak near-unamity",
    "kmm         Kearnes-Marković-McKenzie",
    "n-j         Jónsson chain of length n",
    "n-kk        Kearnes-Kiss chain of length n",
    "n-hmck      HobbyMcKenzie chain of length n",
    "n-hm        Hagemann-Mitschke chain of length n",
    "siggers     Siggers (consider testing for kmm, since it is faster)",
];

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("polymorphism")
        .setting(AppSettings::DeriveDisplayOrder)
        .about("Study the polymorphisms of directed graphs")
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
                .help("List available conditions"),
        )
        .arg(
            Arg::with_name("condition")
                .short("c")
                .long("condition")
                .takes_value(true)
                .value_name("NAME")
                .help("Name of the condition the polymorphism must satisfy [see all conditions with --list]")
                .required_unless("list"),
        )
        .arg(
            Arg::with_name("level-wise")
                .short("L")
                .long("level-wise")
                .help("Test for level-wise satisfiability"),
        )
        .arg(
            Arg::with_name("graph")
                .short("g")
                .long("graph")
                .takes_value(true)
                .value_name("H")
                .help("Search for polymorphisms of graph H [e.g. 0111,00,0 / graph.csv]..."),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .requires("output")
                .takes_value(true)
                .value_name("PATH")
                .help("...or of every graph in file at PATH (one edge-list per line)"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .requires("input")
                .takes_value(true)
                .value_name("PATH")
                .help("...and write the results to file at PATH"),
        )
        .arg(
            Arg::with_name("filter")
                .short("f")
                .long("filter")
                .requires("input")
                .takes_value(true)
                .value_name("PREDICATE")
                .possible_values(&["deny", "admit"])
                .help("Filter graphs from output"),
        )
        .group(
            ArgGroup::with_name("variant")
                .args(&["input", "graph"])
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
    let config = Config::new()
        .level_wise(args.is_present("level-wise"))
        .conservative(args.is_present("conservative"))
        .idempotent(args.is_present("idempotent"));
    let filter = args.value_of("filter").map(|v| match v {
        "deny" => false,
        "admit" => true,
        &_ => unreachable!(),
    });

    if let (Some(input_path), Some(output_path)) = (args.value_of("input"), args.value_of("output"))
    {
        let mut graphs: Vec<AdjMatrix> = Vec::new();
        let lines = std::fs::read_to_string(input_path)?;
        let mut lines = lines.lines();

        if input_path.ends_with(".csv") {
            let _ = lines.next();
        }
        for line in lines {
            graphs.push(from_edge_list(line));
        }

        let log = std::sync::Mutex::new(SearchLog::new());

        println!("  > Checking for polymorphisms...",);
        let start = std::time::Instant::now();

        graphs.into_par_iter().for_each(|item| {
            let problem = create_meta_problem(&item, condition, config).unwrap();
            let mut solver = BTSolver::new(&problem);
            let found = solver.solution_exists();

            if filter.map_or(true, |v| !(v ^ found)) {
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

    let h: AdjMatrix = parse_graph(args.value_of("graph").unwrap())?;
    let problem = create_meta_problem(&h, condition, config)?;
    let mut solver = BTSolver::new(&problem);

    println!("\n> Checking for polymorphisms...");

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

#[derive(Debug, Clone, PartialEq)]
pub struct MPError;

impl std::fmt::Display for MPError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No condition registered with that name")
    }
}

impl std::error::Error for MPError {}

fn create_meta_problem(h: &AdjMatrix, s: &str, config: Config) -> Result<MetaProblem, MPError> {
    match s {
        "majority" => Ok(MetaProblem::new(h, Majority, config)),
        "siggers" => Ok(MetaProblem::new(h, Siggers, config)),
        "kmm" => Ok(MetaProblem::new(h, Kmm, config)),
        _ => {
            if let Some((pr, su)) = s.split_once('-') {
                if let Ok(pr) = pr.parse() {
                    match su {
                        "wnu" => Ok(MetaProblem::new(h, WeakNearUnamity::with_arity(pr), config)),
                        "nu" => Ok(MetaProblem::new(h, NearUnamity::with_arity(pr), config)),
                        // "sigma" => Ok(MetaProblem::new(h, Sigma(pr))),
                        "j" => Ok(MetaProblem::new(h, Jonsson::with_length(pr), config)),
                        "hm" => Ok(MetaProblem::new(
                            h,
                            HagemanMitschke::with_length(pr),
                            config,
                        )),
                        "kk" => Ok(MetaProblem::new(h, KearnesKiss::with_length(pr), config)),
                        "hmck" => Ok(MetaProblem::new(h, HobbyMcKenzie::with_length(pr), config)),
                        "nn" => Ok(MetaProblem::new(h, NoName::with_length(pr), config)),
                        &_ => Err(MPError),
                    }
                } else {
                    Err(MPError)
                }
            } else {
                Err(MPError)
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
    fn new(tree: impl Display, found: bool, stats: &SolveStats) -> Record {
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
        state.serialize_field("ac3_time", &format!("{}s", self.ac3_time.as_secs_f32()))?;
        state.serialize_field("mac3_time", &format!("{}s", self.mac3_time.as_secs_f32()))?;
        state.serialize_field("total_time", &format!("{}s", self.total_time.as_secs_f32()))?;
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

    pub fn add(&mut self, graph: impl Display, found: bool, stats: &SolveStats) {
        self.0.push(Record::new(graph, found, stats));
    }

    pub fn write_csv<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {
        let mut wtr = WriterBuilder::new().has_headers(true).from_path(&path)?;
        for record in &self.0 {
            wtr.serialize(record)?;
        }
        Ok(())
    }
}
