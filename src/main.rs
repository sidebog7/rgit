extern crate clap;
mod cmds;
use clap::{App, Arg};
use cmds::init;

fn main() {
    let matches = App::new("rGit")
        .version("0.1")
        .about("Rust implementation of Git")
        .author("Gareth Pendleton<gareth.sidebottom@gmail.com>")
        .arg(Arg::with_name("init").help("Initialises rGit repository"))
        .arg(Arg::with_name("path").help("File path").multiple(true))
        .get_matches();

    if matches.is_present("init") {
        let mut paths: Vec<&str> = Vec::new();
        if let Some(path_params) = matches.values_of("path") {
            for path in path_params {
                paths.push(path);
            }
        } else {
            paths.push(".");
        }
        init::init(paths);
    }
}
