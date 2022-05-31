use arx::solver::BTSolver;
use clap::{App, Arg, ArgGroup, ArgMatches, SubCommand};
use colored::*;
use itertools::Itertools;
use tripolys::{digraph::AdjMatrix, hcoloring::Instance};

use crate::{parse_graph, CmdResult, print_stats};

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("endomorphism")
        .about("Study the endomorphisms of a graph H")
        .arg(
            Arg::with_name("find")
                .short("f")
                .long("find")
                .help("Find a smallest core of H"),
        )
        .arg(
            Arg::with_name("check")
                .short("c")
                .long("check")
                .help("Check if H is a core"),
        )
        .group(
            ArgGroup::with_name("variant")
                .args(&["find", "check"])
                .required(true),
        )
        .arg(
            Arg::with_name("graph")
                .short("g")
                .long("graph")
                .takes_value(true)
                .value_name("H")
                .required(true)
                .help("The graph to check"),
        )
}

pub fn command(args: &ArgMatches) -> CmdResult {
    let graph = args.value_of("graph").unwrap();
    let h: AdjMatrix = parse_graph(graph)?;

    println!("\n> Checking graph...");
    let problem = Instance::new(h.clone(), h);
    let mut solver = BTSolver::new(&problem);
    let mut sols = Vec::new();
    solver.solve_all(|sol| sols.push(sol));

    let mut injective = true;
    for sol in &sols {
        if !sol.iter().all_unique() {
            injective = false;
            break;
        }
    }
    if injective {
        println!("{}", format!("  ✓ {} is a core\n", graph).green());
    } else {
        println!("{}", format!("  ! {} is not a core\n", graph).red());
        if args.is_present("find") {
            let (_, i) = sols
                .iter()
                .enumerate()
                .map(|(i, sol)| (sol.iter().unique().count(), i))
                .min()
                .unwrap();

            println!("> Homomorphism:");
            for (j, sol) in sols[i].iter().enumerate() {
                println!("  {} -> {}", j, **sol);
            }
        }
    }

    if let Some(stats) = solver.stats() {
        print_stats(stats)
    }
    Ok(())
}
