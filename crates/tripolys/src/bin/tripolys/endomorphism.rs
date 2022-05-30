use clap::{App, Arg, ArgMatches, SubCommand};
use colored::*;
use time::OffsetDateTime;
use tripolys::digraph::AdjMatrix;
use tripolys::tree::is_core_tree;

use crate::{CmdResult, parse_triad};

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("endomorphism")
        .about("Check for an endomorphism of a tree T")
        .arg(
            Arg::with_name("core")
                .short("c")
                .long("core")
                .required_unless("find")
                .help("Check if T is a core"),
        )
        // .arg(
        //     Arg::with_name("find")
        //         .short("f")
        //         .long("find")
        //         .required_unless("core")
        //         .help("Find a smallest core of T"),
        // )
        // .group(
        //     ArgGroup::with_name("variant")
        //         .args(&["find", "core"])
        //         .required(true),
        // )
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
        let tree: AdjMatrix = parse_triad(s)?;

        if args.is_present("core") {
            println!("\n> Checking tree...");
            let tstart = OffsetDateTime::now_utc();
            let result = is_core_tree(&tree);
            let tend = OffsetDateTime::now_utc();

            if result {
                println!("{}", format!("  ✓ {} is a core", s).green());
            } else {
                println!("{}", format!("  ! {} is not a core", s).red());
            }

            println!("\nComputation time: {}s", (tend - tstart).as_seconds_f32());
        }
    }
    Ok(())
}
