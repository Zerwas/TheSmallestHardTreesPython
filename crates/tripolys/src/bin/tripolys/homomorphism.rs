use arx::solver::BTSolver;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use colored::Colorize;
use tripolys::colouring::ColouringProblem;

use crate::{parse_graph, CmdResult};

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
                .required(true),
        )
        .arg(
            Arg::with_name("to")
                .short("t")
                .long("to")
                .value_name("H")
                .takes_value(true)
                .required(true),
        )
        // TODO .arg(
        //     Arg::with_name("output")
        //         .short("o")
        //         .long("output")
        //         .value_name("FILE")
        //         .takes_value(true)
        //         .help("Writes the found homomorphism to file FILE"),
        // )
}

pub fn command(args: &ArgMatches) -> CmdResult {
    let g = parse_graph(args.value_of("from").unwrap())?;
    let h = parse_graph(args.value_of("to").unwrap())?;
    let problem = ColouringProblem::new(&g, &h);

    println!("\n> Checking for homomorphism...");
    let mut solver = BTSolver::new(&problem);

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
