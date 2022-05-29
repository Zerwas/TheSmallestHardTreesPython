use arx::solver::{BTSolver, BTStats};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use colored::*;
use csv::WriterBuilder;
use rayon::prelude::*;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use time::Duration;
use tripolys::digraph::formats::from_edge_list;
use tripolys::digraph::AdjMatrix;

use std::fmt::Display;
use std::path::Path;

use tripolys::algebra::MetaProblem;
use tripolys::algebra::{conditions::*, Config};

use crate::{parse_graph, CmdResult};

const AVAILABLE_CONDITIONS: [&str; 9] = [
    "majority", "siggers", "kkm", "k-wnu", "k-nu", "n-j", "n-hm", "n-kk", "n-hmck",
];

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("polymorphism")
        .setting(AppSettings::DeriveDisplayOrder)
        .about("Study the polymorphisms of finite digraphs")
        .arg(
            Arg::with_name("idempotent")
                .short("I")
                .long("idempotent")
                .help("Require idempotence TODO"),
        )
        .arg(
            Arg::with_name("conservative")
                .short("C")
                .long("conservative")
                .help("Require conservativity TODO"),
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
                .help("The name of the condition the polymorphism must satisfy (see all conditions with --list)")
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
                .help("Check for polymorphisms of graph H"),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .requires("output")
                .takes_value(true)
                .value_name("FILE")
                .help("Check for polymorphisms of each graph in FILE"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .requires("input")
                .takes_value(true)
                .value_name("FILE")
                .help("The name of the file to which the results are written"),
        )
        .arg(
            Arg::with_name("filter")
                .short("f")
                .long("filter")
                .requires("input")
                .takes_value(true)
                .value_name("PREDICATE")
                .possible_values(&["deny", "admit"])
                .help("Filter graphs which deny/admit a polymorphism"),
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

    if let (Some(input_path), Some(output_path)) = (args.value_of("input"), args.value_of("output"))
    {
        let mut graphs: Vec<AdjMatrix> = Vec::new();
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
            // TODO remove clone --------------------------v
            let problem = create_meta_problem(item.clone(), condition, config).unwrap();
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

    let h: AdjMatrix = parse_graph(args.value_of("graph").unwrap())?;
    let problem = create_meta_problem(h, condition, config)?;
    let mut solver = BTSolver::new(&problem);

    println!("\n> Checking for polymorphism...");

    if solver.solution_exists() {
        println!("{}", "  ✓ Exists\n".to_string().green());
    } else {
        println!("{}", "  ! Doesn't exist\n".to_string().red());
    };

    if let Some(stats) = solver.stats() {
        stats.print();
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

fn create_meta_problem(h: AdjMatrix, s: &str, config: Config) -> Result<MetaProblem, MPError> {
    match s {
        "majority" => Ok(MetaProblem::new(h, Majority, config)),
        "siggers" => Ok(MetaProblem::new(h, Siggers, config)),
        "kmm" => Ok(MetaProblem::new(h, Kmm, config)),
        _ => {
            if let Some((pr, su)) = s.split_once('-') {
                if let Ok(pr) = pr.parse() {
                    match su {
                        "wnu" => Ok(MetaProblem::new(h, Wnu(pr), config)),
                        "nu" => Ok(MetaProblem::new(h, Nu(pr), config)),
                        // "sigma" => Ok(MetaProblem::new(h, Sigma(pr))),
                        "j" => Ok(MetaProblem::new(h, Jonsson(pr), config)),
                        "hm" => Ok(MetaProblem::new(h, HagemanMitschke(pr), config)),
                        "kk" => Ok(MetaProblem::new(h, KearnesKiss(pr), config)),
                        "hmck" => Ok(MetaProblem::new(h, HobbyMcKenzie(pr), config)),
                        "nn" => Ok(MetaProblem::new(h, Noname(pr), config)),
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
        state.serialize_field("ac3_time", &format!("{}s", self.ac3_time.as_seconds_f32()))?;
        state.serialize_field(
            "mac3_time",
            &format!("{}s", self.mac3_time.as_seconds_f32()),
        )?;
        state.serialize_field(
            "total_time",
            &format!("{}s", self.total_time.as_seconds_f32()),
        )?;
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

    pub fn add(&mut self, graph: impl Display, found: bool, stats: &BTStats) {
        self.0.push(Record::new(graph, found, stats));
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
