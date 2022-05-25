use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use time::OffsetDateTime;
use tripolys::digraph::formats::to_edge_list;

use std::io::{BufWriter, Write};
use std::path::PathBuf;

use tripolys::tree::generate::{Config, Stats, TreeGenerator};

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
            Arg::with_name("triad")
                .short("t")
                .long("triad")
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
    let triad = args.is_present("triad");
    let core = args.is_present("core");
    let max_arity = if let Some(a) = args.value_of("max_arity") {
        a.parse::<usize>()?
    } else {
        end
    };

    let config = Config {
        max_arity,
        core,
        triad,
        start,
        end,
        stats: Some(Stats::default()),
    };

    let mut generator = TreeGenerator::with_config(config);

    for num_vertices in start..=end {
        println!("\n> #vertices: {}", num_vertices);
        println!("  > Generating trees...");
        let start = OffsetDateTime::now_utc();
        let trees = generator.next();
        let end = OffsetDateTime::now_utc();
        println!("    - total_time: {}s", (end - start).as_seconds_f32());
        let dir_name = if num_vertices < 10 {
            String::from("0") + &num_vertices.to_string()
        } else {
            num_vertices.to_string()
        };
        let mut path = PathBuf::from(data_path).join(dir_name);
        if triad {
            path.push("triads");
        }
        std::fs::create_dir_all(&path)?;
        let file_name = if core { "cores.edges" } else { "trees.edges" };
        let mut writer = BufWriter::new(std::fs::File::create(path.join(file_name))?);

        for tree in trees {
            to_edge_list(&tree, &mut writer)?;
            writer.write("\n".as_bytes())?;
        }
    }

    Ok(())
}
