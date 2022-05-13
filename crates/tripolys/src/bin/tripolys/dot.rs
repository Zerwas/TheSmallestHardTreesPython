use clap::{App, Arg, ArgMatches, SubCommand};
use tripolys::digraph::to_dot;

use crate::{parse_graph, CmdResult};

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("dot")
        .about("Convert the input graph to dot format")
        .arg(
            Arg::with_name("graph")
                .short("G")
                .long("graph")
                .takes_value(true)
                .value_name("GRAPH")
                .help("The graph to print"),
        )
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .takes_value(true)
                .value_name("FILE")
                .help("Name of the output file"),
        )
}

pub fn command(args: &ArgMatches) -> CmdResult {
    let tree = parse_graph(args.value_of("graph").unwrap())?;
    let mut f = std::fs::File::create(args.value_of("out").unwrap())?;
    to_dot(&tree.into(), &mut f)?;

    Ok(())
}
