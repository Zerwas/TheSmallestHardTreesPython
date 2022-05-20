use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use itertools::Itertools;

use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Instant;

use tripolys::digraph::ToGraph;
use tripolys::tree::generate::{Config, Generator};
use tripolys::tree::Node;

use crate::CmdResult;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("generate")
        .setting(AppSettings::DeriveDisplayOrder)
        .about("Generate trees with a given number of vertices")
        .arg(
            Arg::with_name("core")
                .short("core")
                .long("core")
                .help("Whether the generated graphs should be cores"),
        )
        .arg(
            Arg::with_name("triads")
                .short("t")
                .long("triads")
                .help("Generate triads"),
        )
        .arg(
            Arg::with_name("start")
                .short("s")
                .long("start")
                .takes_value(true)
                .value_name("NUM")
                .help("Number of nodes to start at"),
        )
        .arg(
            Arg::with_name("end")
                .short("e")
                .long("end")
                .takes_value(true)
                .value_name("NUM")
                .help("Number of nodes to end at (inclusive)"),
        )
        .arg(
            Arg::with_name("data_path")
                .short("d")
                .long("data_path")
                .value_name("PATH")
                .takes_value(true)
                .default_value("./data")
                .help("Path of the data directory"),
        )
        .arg(
            Arg::with_name("max_arity")
                .short("m")
                .long("max_arity")
                .takes_value(true)
                .value_name("NUM")
                .conflicts_with("triads")
                .help("The maximal arity of the trees"),
        )
}

pub fn command(args: &ArgMatches) -> CmdResult {
    let data_path = args.value_of("data_path").unwrap();

    let start = args.value_of("start").unwrap().parse::<usize>()?;
    let end = args.value_of("end").unwrap().parse::<usize>()?;
    let triads = args.is_present("triads");
    let core = args.is_present("core");
    let max_arity = if let Some(a) = args.value_of("max_arity") {
        a.parse::<usize>()?
    } else {
        end
    };

    let config = Config {
        max_arity,
        core,
        triads,
    };

    let mut generator = Generator::with_config(config);

    for order in start..=end {
        println!("\n> #vertices: {}", order);
        println!("  > Generating trees...");
        let start = Instant::now();
        let trees = generator.resume(order)?;
        println!("    - total_time: {:?}", start.elapsed());
        let order_dir = if order < 10 {
            String::from("0") + &order.to_string()
        } else {
            order.to_string()
        };
        let mut path = PathBuf::from(data_path).join(order_dir);
        if triads {
            path.push("triads");
        }
        std::fs::create_dir_all(&path)?;
        let file_name = if core { "cores.edges" } else { "trees.edges" };
        let mut writer = BufWriter::new(std::fs::File::create(path.join(file_name))?);

        for tree in &trees {
            let _ = writer.write(format!("{}\n", tree.to_graph()).as_bytes());
        }
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegistryError;

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No Polymorphism registered with name")
    }
}

impl std::error::Error for RegistryError {}

fn from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Node>, std::io::Error> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|l| Node::from_str(l).unwrap())
        .collect())
}
