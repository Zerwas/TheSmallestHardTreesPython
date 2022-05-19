use std::str::FromStr;
use std::time::Instant;

use clap::{App, Arg, ArgGroup, ArgMatches, SubCommand};
use colored::*;
use tripolys::tree::{is_core_tree, Node, Triad};

use crate::CmdResult;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("endomorphism")
        .about("Check for an endomorphism of H")
        .arg(
            Arg::with_name("core")
                .short("c")
                .long("core")
                .required_unless("find")
                .help("Check if H is a core"),
        )
        .arg(
            Arg::with_name("find")
                .short("f")
                .long("find")
                .required_unless("core")
                .help("Find a smallest core of H"),
        )
        .group(
            ArgGroup::with_name("variant")
                .args(&["find", "core"])
                .required(true),
        )
        .arg(
            Arg::with_name("tree")
                .short("t")
                .long("tree")
                .takes_value(true)
                .value_name("T")
                .help("Check polymorphism on T"),
        )
}

pub fn command(args: &ArgMatches) -> CmdResult {
    if let Some(s) = args.value_of("tree") {
        let tree: Node = if let Ok(triad) = Triad::from_str(s) {
            triad.into()
        } else {
            Node::from_str(s)?
        };

        if args.is_present("core") {
            println!("\n> Checking tree...");
            let start = Instant::now();
            let result = is_core_tree(&tree);
            let time = start.elapsed();

            if result {
                println!("{}", format!("  ✓ {} is a core", s).green());
            } else {
                println!("{}", format!("  ✘ {} is not a core", s).red());
            }

            println!("\nComputation time: {:?}", time);
        }
    }
    Ok(())
}
