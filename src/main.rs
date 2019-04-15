extern crate clap;
mod cmds;
use clap::{App, Arg, SubCommand, Values};
use cmds::{commit, init};

fn main() {
    let matches = App::new("rGit")
        .version("0.1")
        .about("Rust implementation of Git")
        .author("Gareth Pendleton<gareth.sidebottom@gmail.com>")
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialises rGit repository")
                .arg(Arg::with_name("path").help("File path").multiple(true)),
        )
        .subcommand(
            SubCommand::with_name("commit")
                .about("Commits given files")
                .arg(
                    Arg::with_name("path")
                        .help("File path")
                        .multiple(true)
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("init", Some(sub_com)) => {
            let paths = get_paths(sub_com.values_of("path"), Some("."));

            init::init(paths);
        }
        ("commit", Some(sub_com)) => {
            let paths = get_paths(sub_com.values_of("path"), None);

            commit::commit(paths);
        }
        _ => {}
    }
}

fn get_paths<'a>(paths_str: Option<Values<'a>>, empty_option: Option<&'a str>) -> Vec<&'a str> {
    let mut paths: Vec<&'a str> = Vec::new();
    if let Some(path_params) = paths_str {
        for path in path_params {
            paths.push(path);
        }
    } else {
        match empty_option {
            Some(empty_str) => {
                paths.push(empty_str);
            }
            None => {
                panic!("Error with commit missing files");
            }
        }
    }
    paths
}
