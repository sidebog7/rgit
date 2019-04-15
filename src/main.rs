extern crate clap;
mod cmds;
use clap::{App, Arg, SubCommand};
use cmds::init;

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
        .get_matches();

    if let Some(sub_com) = matches.subcommand_matches("init") {
        let mut paths: Vec<&str> = Vec::new();
        if let Some(path_params) = sub_com.values_of("path") {
            for path in path_params {
                paths.push(path);
            }
        } else {
            paths.push(".");
        }
        init::init(paths);
    }
}
